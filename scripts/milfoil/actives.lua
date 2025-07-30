-- Active elements in the world.
local module = {};

module.operator = {
	position = "root",
	-- besides = {},
	size = 2.0,
};
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
