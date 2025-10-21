local module = {};

require("app_aliases");
require("milfoil/registers");

local app = require("app");
local obj = require("objects");
local tree = require("hierarchy");

local actives = require("milfoil/actives");

local shall_init = true;
function module.init()
	local pos_begin = tree:reach("root", "milfoil-staff_lounge", tree.NODE_CLONE);
	actives.operator:relocate(pos_begin);
	app.queue.push_plain(app.i18n.t("plots-introduction-a1"));
end

function module.routine()
	if shall_init then
		module.init();
		shall_init = false;
	end
	app.actions:clear();
	local pos = actives.operator.position;
	obj.apply(tree.nodes[actives.operator.position], "setup_actions");
	for id, _ in pairs(tree:get_children_id(pos)) do
		local node = tree.nodes[id];
		obj.apply(node, "setup_actions");
	end
	app.queue.flush();
end

function app.update()
	xpcall(module.routine, function(err)
		print(err);
	end);
end

app.update();

return module;
