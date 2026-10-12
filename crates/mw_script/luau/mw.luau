--!nonstrict
-- The `mw` runtime skeleton: the Luau half of the translated-script runtime
-- (scripting-adapter.md "The `mw` runtime library"; built in M5 W2, grown in W3).
--
-- `Runtime.new(host, functions)` returns the `mw` table every generated module receives:
--   host       W3's Rust `@mw/script` side: global functions and handle methods by snake_case
--              name (`host.functions.menu_mode(self)`, `host.methods.get_pos(handle, "z")`),
--              `host.vars_of(target)` (an object's or global script's variable table and its
--              types), `host.handle(id)` (a reference handle). Anything absent falls back to the
--              function table's neutral value, so this skeleton runs every module without a world.
--   functions  generated from functions.txt (`mw_script::emit::runtime_functions`):
--              name -> { status = "live"|"stub"|"later", neutral = n, ref = bool }.
-- No state lives in globals (sandboxed states share one writable environment, m5-l1).

local Runtime = {}

local function trunc(x: number): number
	if x ~= x or x == math.huge or x == -math.huge then
		return 0
	end
	if x >= 0 then
		return math.floor(x)
	end
	return math.ceil(x)
end

-- Assignment conversions (emit.rs "Numeric semantics"; pending the OpenMW oracle).
local function short(x: number): number
	local n = trunc(x) % 65536
	if n >= 32768 then
		n -= 65536
	end
	return n
end

local function long(x: number): number
	local n = trunc(x) % 4294967296
	if n >= 2147483648 then
		n -= 4294967296
	end
	return n
end

local function float(x: number): number
	if x ~= x then
		return x
	end
	return (string.unpack("<f", string.pack("<f", x)))
end

local convert = { short = short, long = long, float = float }

function Runtime.new(host, functions)
	host = host or {}
	local host_functions = host.functions or {}
	local host_methods = host.methods or {}
	local mw = {}
	mw.short, mw.long, mw.float = short, long, float
	mw.functions = functions
	-- Calls that reached a stand-in, by name (dev overlay and the W3 runtime report).
	mw.fallback_calls = {}

	local function fallback(name: string)
		local row = functions[name]
		local neutral = if row then row.neutral else 0
		local counts = mw.fallback_calls
		return function()
			counts[name] = (counts[name] or 0) + 1
			return neutral
		end
	end

	-- Globals (GLOB): unset reads are 0; W3 fills them from the records and saves them.
	mw.globals = setmetatable({}, { __index = function()
		return 0
	end })
	function mw.get_global(name: string): number
		return mw.globals[name]
	end
	function mw.set_global(name: string, value: number)
		mw.globals[name] = float(value)
	end

	-- Handles: methods are host methods or stand-ins; one handle per ID.
	local Handle = {}
	Handle.__index = function(_, name)
		local f = host_methods[name] or fallback(name)
		Handle[name] = f
		return f
	end
	local refs = {}
	function mw.ref(id: string)
		local h = refs[id]
		if h == nil then
			h = if host.handle then host.handle(id) else nil
			if h == nil then
				h = { id = id }
			end
			setmetatable(h, Handle)
			refs[id] = h
		end
		return h
	end
	function mw.instance(id: string, vars, types)
		return setmetatable({ id = id, vars = vars, types = types }, Handle)
	end

	-- Other objects' and the speaker's variables. Unknown targets read 0; writes are dropped.
	local function vars_of(target)
		if type(target) == "table" then
			return target.vars, target.types
		end
		if host.vars_of then
			return host.vars_of(target)
		end
		return nil, nil
	end
	function mw.get_var(target, name: string): number
		local vars = vars_of(target)
		return if vars then (vars[name] or 0) else 0
	end
	function mw.set_var(target, name: string, value: number)
		local vars, types = vars_of(target)
		if vars and types and types[name] then
			vars[name] = convert[types[name]](value)
		end
	end
	function mw.discard(_value) end

	-- Module constructors.
	function mw.script(def)
		return def
	end
	function mw.dialogue(def)
		return def
	end
	-- A fresh instance variable table for a script definition: every local zero.
	function mw.new_vars(def)
		local vars = {}
		for name in def.vars do
			vars[name] = 0
		end
		return vars
	end

	-- Global functions this file implements (status live, no world needed).
	local own = {}
	function own.random(_self, n: number): number
		n = trunc(n)
		return if n > 0 then math.random(0, n - 1) else 0
	end
	function own.get_square_root(_self, x: number): number
		return if x > 0 then math.sqrt(x) else 0
	end

	return setmetatable(mw, { __index = function(t, name)
		local f = own[name] or host_functions[name] or fallback(name)
		rawset(t, name, f)
		return f
	end })
end

return Runtime
