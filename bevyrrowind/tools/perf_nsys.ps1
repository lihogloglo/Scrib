# GPU time of every wgpu pass (render and compute) with NVIDIA Nsight Systems.
#
#   powershell -NoProfile -ExecutionPolicy Bypass -File tools/perf_nsys.ps1 -Name balmora1
#       [-Scenario balmora] [-Resolution 1280x720] [-Delay 35] [-Duration 8]
#       [-Off terrain,shadows] [-Exe target/profiling/bevyrrowind.exe] [-Cpu] [-DryRun]
#
# Runs a suite scenario (`bevyrrowind perf list`) under `nsys profile` with
# WGPU_DEBUG=1, so wgpu emits a Vulkan debug label per pass; records
# `Duration` seconds after `Delay` (inside the measured window: standing
# scenarios are stretched to Delay + Duration + 15 s), then writes, under
# target/perf/nsys/:
#   <Name>.nsys-rep                       the capture (open in the Nsight GUI)
#   <Name>_vulkan_gpu_marker_sum.csv      GPU time per pass label (nsys stats)
#   <Name>-passes.md                      per-frame table (bevyrrowind perf nsys)
#   <Name>/                               the run's sf_diag report (frame times, hitches)
#
# nsys registers its Vulkan layer in the registry, so the capture must run
# elevated: the script asks once (UAC prompt; someone must be at the
# machine), runs `nsys profile` in the elevated window, and does the rest
# here. Nothing else needs admin. Like the suite, it waits until no
# cargo/rustc/bevyrrowind process runs. CPU sampling is off unless -Cpu
# (it makes the import slow and Rust symbols are not resolved on Windows).
param(
    [Parameter(Mandatory = $true)][string]$Name,
    [string]$Scenario = 'balmora',
    [string]$Resolution = '1280x720',
    [int]$Delay = 35,
    [int]$Duration = 8,
    [string]$Off = '',
    [string]$Exe = '',
    [switch]$Cpu,
    [switch]$DryRun,
    # Internal: set on the elevated re-launch.
    [switch]$Capture
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
$nsys = 'C:\Program Files\NVIDIA Corporation\Nsight Systems 2026.5.1\target-windows-x64\nsys.exe'
if (-not $Exe) { $Exe = Join-Path $root 'target\profiling\bevyrrowind.exe' }
$out = Join-Path $root 'target\perf\nsys'
$rep = Join-Path $out $Name

if (-not (Test-Path $nsys)) { throw "Nsight Systems not found at $nsys" }
if (-not (Test-Path $Exe)) { throw "$Exe not built: cargo build --profile profiling -p bevyrrowind" }

# The game's arguments: the scenario, plus a window long enough to capture in.
$gameArgs = @(& $Exe perf args $Scenario)
if ($LASTEXITCODE -ne 0) { throw "unknown scenario $Scenario (see: $Exe perf list)" }
$gameArgs += @('--resolution', $Resolution, '--bench-out', $rep, '--bench-baseline', 'none')
$scenarioFile = $gameArgs[[array]::IndexOf($gameArgs, '--bench') + 1]
if ((Get-Content -Raw $scenarioFile) -match '"kind"\s*:\s*"duration"') {
    $gameArgs += @('--bench-seconds', "$($Delay + $Duration + 15)")
}
if ($Off) { $gameArgs += @('--off', $Off) }

$sample = if ($Cpu) {
    @('--sample=process-tree', '--sampling-frequency=4000', '--cpuctxsw=process-tree')
} else { @('--sample=none', '--cpuctxsw=none') }
$profileArgs = @('profile', '--trace=vulkan,vulkan-annotations', '--vulkan-gpu-workload=individual') + $sample +
    @("--delay=$Delay", "--duration=$Duration", '--force-overwrite=true', '-o', $rep, $Exe) + $gameArgs

function Quote([string[]]$a) { ($a | ForEach-Object { if ($_ -match '[\s,"]') { '"' + ($_ -replace '"', '\"') + '"' } else { $_ } }) -join ' ' }

if ($Capture) {
    # Elevated: capture only.
    $env:WGPU_DEBUG = '1'
    # Where nsys finds bevyrrowind.pdb to name sampled functions (-Cpu).
    $env:_NT_SYMBOL_PATH = Split-Path -Parent $Exe
    New-Item -ItemType Directory -Force $out | Out-Null
    & $nsys @profileArgs *> "$rep-nsys.log"
    exit $LASTEXITCODE
}

if ($DryRun) {
    Write-Output "elevated: WGPU_DEBUG=1 `"$nsys`" $(Quote $profileArgs)"
    Write-Output "then: `"$nsys`" stats --report vulkan_gpu_marker_sum --format csv --force-overwrite=true -o $rep $rep.nsys-rep"
    Write-Output "then: $Exe perf nsys ${rep}_vulkan_gpu_marker_sum.csv --out $rep-passes.md"
    exit 0
}

# A quiet machine, as the suite requires.
while ($true) {
    $busy = Get-Process -ErrorAction SilentlyContinue | Where-Object { $_.ProcessName -in @('cargo', 'rustc', 'bevyrrowind', 'ovid') }
    if (-not $busy) { break }
    Write-Output "waiting: $((($busy | Select-Object -ExpandProperty ProcessName -Unique) -join ', ')) running"
    Start-Sleep -Seconds 10
}

$self = @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $PSCommandPath, '-Name', $Name, '-Scenario', $Scenario,
    '-Resolution', $Resolution, '-Delay', $Delay, '-Duration', $Duration, '-Exe', $Exe, '-Capture')
if ($Off) { $self += @('-Off', $Off) }
if ($Cpu) { $self += '-Cpu' }
Write-Output "capturing $Scenario at $Resolution ($Delay s delay, $Duration s); approve the UAC prompt"
$p = Start-Process powershell -Verb RunAs -Wait -PassThru -ArgumentList (Quote $self)
if (-not (Test-Path "$rep.nsys-rep")) { throw "no capture (exit $($p.ExitCode)); see $rep-nsys.log" }

& $nsys stats --report vulkan_gpu_marker_sum --format csv --force-overwrite=true -o $rep "$rep.nsys-rep" *> "$rep-stats.log"
$csv = "${rep}_vulkan_gpu_marker_sum.csv"
if (-not (Test-Path $csv)) { throw "nsys stats wrote no $csv; see $rep-stats.log" }
& $Exe perf nsys $csv --out "$rep-passes.md"
Write-Output "`nwrote $rep-passes.md (frame times of the same run: $rep\)"
