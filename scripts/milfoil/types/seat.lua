return {
	actions = function(args)
		local actives = require("milfoil/actives");

		if actives.operator:isin(args.object.id) then
			return {};
		end

		local obj = require("objects");
		local function sit()
			actives.time:tick(5.0);
			actives.operator:relocate(args.object.id);
			obj.call(actives.operator.components.stamina, "mutate", -1.0);
			obj.call(actives.operator.components.stamina, "mutate_speed", 0.5);
		end

		return { ["milfoil-sit"] = sit };
	end,
}
