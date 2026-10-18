--!strict
-- An entity script: one instance per placement it is attached to, with its own plain-data state
-- (saved with the mod, kept across hot reloads and cell unloads).
local core = require("@sf/core")

local lamp = {}

-- Offered before the mod's global "activated" handlers. Every second use is "handled": the
-- engine's default action (open, pick up, ...) doesn't run.
function lamp.activated(h: Handle, _payload: any, state: { [any]: any })
	state.uses = (state.uses or 0) + 1
	core.log("lamp", h:record(), "used", state.uses, "times")
	if state.uses % 2 == 0 then
		return "handled"
	end
	return nil
end

-- Every Update slot while the placement is loaded.
function lamp.tick(_h: Handle, dt: number, state: { [any]: any })
	state.age = (state.age or 0) + dt
end

return lamp
