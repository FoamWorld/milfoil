local module = {};

require("app_aliases");
require("hierarchy");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");
for _, value in ipairs({ "bar", "item_supervision_screen", "physical", "seat" }) do
	obj.declare("milfoil-" .. value, require("milfoil/types/" .. value));
end

require("milfoil/registers");

local actives = require("milfoil/actives");

local tree = require("hierarchy");
tree.resolvers["milfoil"] = function(name)
	local func = require("milfoil/objects/" .. name);
	if type(func) ~= "function" then
		error("Target id \"" .. name .. "\" failed to resolve to a function.", 1);
	end
	return func();
end

local shall_init = true;
function module.init()
	local pos_begin = tree:reach("root", "milfoil-staff_lounge", tree.NODE_CLONE);
	actives.operator:relocate(pos_begin);
	app.queue.push_plain(app.i18n.t("milfoil-plot-introduction-1"));
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
