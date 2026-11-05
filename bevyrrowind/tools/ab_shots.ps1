# Owner A/B screenshots: runs each before/after pair with `--screenshot` (the game opens, waits until
# loaded, saves, exits), then writes and opens a side-by-side page with a drag slider.
# Owner-run only (agents never launch the game, owner 2026-11-01). Build first:
#   cargo build -p bevyrrowind --features dev
#   powershell -File bevyrrowind/tools/ab_shots.ps1            # all pairs
#   powershell -File bevyrrowind/tools/ab_shots.ps1 -Only delight-office,reach-office
# Output: shots/ab-<date>/ (untracked). Edit $pairs below for other checks.

param([string[]]$Only = @(), [string]$Resolution = '1920x1080', [int]$Settle = 5)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\debug\bevyrrowind.exe'
if (-not (Test-Path $exe)) { throw "no dev build at $exe (cargo build -p bevyrrowind --features dev)" }
$sysroot = (& rustc --print sysroot).Trim()
$env:PATH = "$sysroot\bin;$root\target\debug\deps;$root\target\debug;$env:PATH"

$office = @('view', 'Seyda Neen, Census and Excise Office')
$balmora = @('view', 'Balmora')

# a = before (old), b = after (new). Each run gets the common flags appended.
$pairs = @(
    @{ id = 'delight-office'; check = 'TX-delight'; title = 'De-lit textures, Census office'
       look = 'Corners and edges no longer double-dark; walls not flat or cartoony; no colour shift; no blocky compression.'
       a = $office + @('--off', 'delight'); b = $office; al = 'original textures'; bl = 'de-lit (strength 0.5)' },
    @{ id = 'delight-balmora'; check = 'TX-delight'; title = 'De-lit textures, Balmora 14:00'
       look = 'Walls and roofs under sunlight: painted shading no longer doubled; foliage not thinner far away.'
       a = $balmora + @('--hour', '14', '--off', 'delight'); b = $balmora + @('--hour', '14'); al = 'original textures'; bl = 'de-lit' },
    @{ id = 'reach-office'; check = 'LT-reach'; title = 'Lamp reach, Census office'
       look = 'No lamp light on the far side of walls; pools fade without a ring; rooms not too dark (else try 2.4).'
       a = $office + @('--range-unshadowed', '4'); b = $office; al = 'old reach (4 x radius)'; bl = 'new: unshadowed 1.5 x' },
    @{ id = 'reach-balmora-night'; check = 'LT-reach'; title = 'Lamp reach, Balmora 22:00'
       look = 'Nearby unshadowed lamps no longer paint facades across the street.'
       a = $balmora + @('--hour', '22', '--range-unshadowed', '4'); b = $balmora + @('--hour', '22'); al = 'old reach'; bl = 'new reach' },
    @{ id = 'gi-office'; check = 'GI-lampbody'; title = 'Probe GI, Census office (lamp bodies, ambient floor 0.1)'
       look = 'Warm bounce near lamps without hot spots on the lamp housings; dark corners not pitch black.'
       a = $office + @('--off', 'gi'); b = $office; al = 'GI off (flat ambient)'; bl = 'GI on' },
    @{ id = 'all-office'; check = 'all'; title = 'Everything from 2026-11-05, Census office'
       look = 'Overall: old lighting and textures vs new reach + de-lit textures (GI on in both).'
       a = $office + @('--off', 'delight', '--range-unshadowed', '4'); b = $office; al = 'before'; bl = 'after' }
)
if ($Only.Count -gt 0) { $pairs = $pairs | Where-Object { $Only -contains $_.id } }

$out = Join-Path (Split-Path -Parent $root) ("shots\ab-" + (Get-Date -Format 'yyyy-MM-dd-HHmm'))
New-Item -ItemType Directory -Force $out | Out-Null

function Invoke-Shot([string[]]$argv, [string]$name) {
    $png = Join-Path $out "$name.png"
    $all = $argv + @('--resolution', $Resolution, '--settle', "$Settle", '--screenshot', $png, '--screenshot-anyway')
    $quoted = $all | ForEach-Object { if ($_ -match '\s') { '"' + $_ + '"' } else { $_ } }
    Write-Host "  $name : $($all -join ' ')"
    $p = Start-Process -FilePath $exe -ArgumentList $quoted -WorkingDirectory $root -PassThru -Wait `
        -RedirectStandardOutput (Join-Path $out "$name.out.log") -RedirectStandardError (Join-Path $out "$name.err.log")
    $note = ''
    foreach ($log in @("$name.out.log", "$name.err.log")) {
        $hit = Select-String -Path (Join-Path $out $log) -Pattern 'delight: (layer mounted|off)[^\r\n]*' -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($hit) { $note = $hit.Matches[0].Value }
    }
    if ($p.ExitCode -ne 0 -or -not (Test-Path $png)) { $note = "FAILED (exit $($p.ExitCode), see $name.err.log) $note" }
    return $note
}

Write-Host "Baking (warm bake is under a second)..."
$bake = Start-Process -FilePath $exe -ArgumentList 'bake' -WorkingDirectory $root -PassThru -Wait `
    -RedirectStandardOutput (Join-Path $out 'bake.out.log') -RedirectStandardError (Join-Path $out 'bake.err.log')
if ($bake.ExitCode -ne 0) { throw "bake failed, see $out\bake.err.log" }

$cards = foreach ($pr in $pairs) {
    Write-Host $pr.title
    $na = Invoke-Shot $pr.a "$($pr.id)-a"
    $nb = Invoke-Shot $pr.b "$($pr.id)-b"
    @"
<section class="card">
  <h2>$($pr.title) <span class="id">$($pr.check)</span></h2>
  <p class="look">$($pr.look)</p>
  <div class="cmp" data-split="50">
    <img src="$($pr.id)-a.png" alt="before"><div class="after"><img src="$($pr.id)-b.png" alt="after"></div>
    <div class="bar"></div><span class="lab l">$($pr.al)</span><span class="lab r">$($pr.bl)</span>
  </div>
  <p class="notes">before: $na &nbsp; after: $nb</p>
</section>
"@
}

$html = @"
<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>A/B Shots</title>
<style>
:root { --bg:#111; --fg:#ddd; --mut:#999; --acc:#e8b04a; }
body { background:var(--bg); color:var(--fg); font:14px/1.45 system-ui,sans-serif; margin:0 16px 40px; }
h1 { font-size:18px; } h2 { font-size:15px; margin:0 0 4px; } .id { color:var(--acc); font-weight:normal; margin-left:8px; }
.card { max-width:1600px; margin:24px auto; } .look, .notes { color:var(--mut); margin:2px 0 8px; } .notes { font-size:12px; }
.cmp { position:relative; user-select:none; cursor:ew-resize; line-height:0; }
.cmp img { width:100%; display:block; } .after { position:absolute; inset:0; overflow:hidden; }
.after img { position:absolute; top:0; left:0; } .bar { position:absolute; top:0; bottom:0; width:2px; background:var(--acc); }
.lab { position:absolute; top:8px; background:#000a; padding:4px 8px; line-height:1.2; font-size:12px; } .l { left:8px; } .r { right:8px; }
</style></head><body>
<h1>A/B shots, $(Get-Date -Format 'yyyy-MM-dd HH:mm')</h1>
<p class="look">Drag across an image (or press 1 / 2 / 3 for before / split / after on the one under the mouse). Left = before, right = after.</p>
$($cards -join "`n")
<script>
document.querySelectorAll('.cmp').forEach(function (c) {
  var after = c.querySelector('.after'), img = after.querySelector('img'), bar = c.querySelector('.bar');
  function set(p) { p = Math.max(0, Math.min(100, p)); after.style.clipPath = 'inset(0 0 0 ' + p + '%)'; bar.style.left = p + '%'; c.dataset.split = p; }
  function size() { img.style.width = c.clientWidth + 'px'; }
  window.addEventListener('resize', size); c.querySelector('img').addEventListener('load', size); size(); set(50);
  function at(e) { var r = c.getBoundingClientRect(); set((e.clientX - r.left) / r.width * 100); }
  c.addEventListener('pointerdown', function (e) { c.setPointerCapture(e.pointerId); at(e); });
  c.addEventListener('pointermove', function (e) { if (e.buttons) at(e); });
  c.addEventListener('mouseenter', function () { window.__cmp = set; });
});
document.addEventListener('keydown', function (e) { if (!window.__cmp) return;
  if (e.key === '1') window.__cmp(100); if (e.key === '2') window.__cmp(50); if (e.key === '3') window.__cmp(0); });
</script></body></html>
"@
$page = Join-Path $out 'index.html'
$html | Out-File -FilePath $page -Encoding utf8
Write-Host "Done: $page"
Start-Process $page
