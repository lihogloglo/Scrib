--!strict
-- Script API v0.1 tour (superfargoth docs/specs/scripting.md "Curated API"): every @sf module once.
-- Activate something: it gets the "lamp" entity script, and a copy of it appears 1 m above it for
-- three seconds.
local core = require("@sf/core")
local events = require("@sf/events")
local timers = require("@sf/timers")
local world = require("@sf/world")
local interfaces = require("@sf/interfaces")

-- Top level: registration only. The world isn't readable here (no slot is open); use on_load.
local s = core.storage()
s.spawned = s.spawned or 0

world.define_script("lamp", require("./lamp"))

interfaces.export("tour", {
	spawned = function()
		return s.spawned
	end,
})

timers.register("despawn", function(p)
	core.log("despawning the copy:", world.despawn(p.copy))
end)

-- Runs in the first Update slot after start, hot reload or save load: world reads work here.
core.on_load(function(info)
	s.last_load = info.reason
	s.player_x = world.position(world.player())
	core.log("on_load", info.reason, "version", info.version, "player at x =", s.player_x)
end)

events.on("activated", function(e)
	local target = e.target
	local record = target:record()
	if record == nil then
		return nil
	end
	if target:script_of() == nil then
		target:attach("lamp")
	end
	local x, y, z = target:position()
	local copy = world.spawn(record, { x = x, y = y + 1, z = z, worldspace = target:worldspace() })
	s.spawned += 1
	timers.after(3, "despawn", { copy = copy })
	local near = world.nearby(target, 2)
	core.log("activated", record, "-> spawned a copy;", #near, "placements within 2 m")
	return nil
end)
