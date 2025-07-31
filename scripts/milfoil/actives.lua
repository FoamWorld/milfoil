-- Active elements in the world.
local module = {};

local bar = require("milfoil/types/bar");

module.operator = {
	position = "root",
	-- besides = {},
	size = 2.0,
	components = {
		health = bar.new(10.0),
		stamina = bar.new(200.0, 0.1),
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
