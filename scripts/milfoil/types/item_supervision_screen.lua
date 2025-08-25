local app = require("app");
local actives = require("milfoil/actives");
local obj = require("objects");
local tree = require("hierarchy");

local function watch()
	local pos = actives.operator.position;
	local children = tree:get_children_id(pos);
	for id, _ in pairs(children) do
		local node = tree.nodes[id];
		-- todo: 按照记录；计数
		app.queue.push_plain(obj.apply(node, "name"));
	end
end

return {
	actions = function(args)
		return {
			["milfoil-watch"] = watch
		};
	end,
}
