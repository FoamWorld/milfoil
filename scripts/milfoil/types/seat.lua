local actives = require("milfoil/actives");
local obj = require("objects");
local tree = require("hierarchy");

local function sit(args)
	actives.time:tick(5.0);
	actives.operator:relocate(args.object.id);
	obj.call(actives.operator.components.stamina, "mutate", -1.0);
	obj.call(actives.operator.components.stamina, "mutate_speed", 0.5);
end

local function leave(args)
	actives.time:tick(5.0);
	actives.operator:relocate(tree:get_parent_id(args.object.id));
	obj.call(actives.operator.components.stamina, "mutate", -1.0);
end

return {
	actions = function(args)
		if actives.operator:isin(args.object.id) then
			return { ["milfoil-leave"] = function() leave(args) end };
		else
			return { ["milfoil-sit"] = function() sit(args) end };
		end
	end,
}
