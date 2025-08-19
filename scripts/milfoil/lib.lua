local module = {};

require("app_aliases");
require("hierarchy");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");
for _, value in ipairs({ "bar", "physical", "seat" }) do
	obj.declare("milfoil-" .. value, require("milfoil/types/" .. value));
end

require("milfoil/registers");

local actives = require("milfoil/actives");

Tree:add_child("#1", require("milfoil/objects/staff_lounge"), nil);
Tree:add_child("#2", require("milfoil/objects/sofa"), "#1");
actives.operator:relocate("#1");

app.queue.push(app.i18n.t("milfoil-plot-introduction-1"));

function module.routine()
	app.actions:clear();
	local set = Tree.get_children_id(actives.operator.position);
	for id, _ in pairs(set) do
		local node = Tree.nodes[id];
		obj.apply(node, "setup_actions");
	end
	local node = Tree.nodes[actives.operator.position];
	obj.apply(node, "setup_actions");
end

function app.update()
	xpcall(module.routine, function(err)
		print(err);
	end);
end

app.update();

return module;
