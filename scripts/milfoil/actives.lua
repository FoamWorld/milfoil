-- Active elements in the world.
local module = {};

local obj = require("objects");

local function bar(max, speed)
	return obj.new():with("core-bar", {
		max = max,
		speed = speed,
	});
end

module.operator = {
	position = "root",
	-- besides = {},
	size = 2.0,
	components = {
		health = bar(10.0),
		stamina = bar(200.0, 0.1),
	},
};

function module.operator:relocate(id)
	self.position = id;
end

function module.operator:isin(id)
	return self.position == id;
end

function module.operator:update()
	-- todo
end

module.time = {
	locked = false,
	value = 0.0,
}

function module.time:update()
	-- do nothing
end

function module.time:tick(delta)
	if not self.locked then
		self.value = self.value + delta;
	end
end

return module;
