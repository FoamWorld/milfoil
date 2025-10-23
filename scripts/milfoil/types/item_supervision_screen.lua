local app = require("app");
local tree = require("hierarchy");

local function list_notes(object_id)
	for _, value in ipairs(tree.nodes[object_id].notes) do
		app.queue.push_plain(value);
	end
end

local function list_objects(object_id)
	-- local pos = actives.operator.position;
	-- local children = tree:get_children_id(pos);
	local marked = "记录的物品列表：\n";
	for _, value in pairs(tree.nodes[object_id].records) do
		local name = app.i18n._("obj-" .. value.name) or "未知的物品";
		local num = value.num or 1;
		marked = marked .. "* " .. name .. " (" .. num .. ")\n";
	end

	marked = marked .. "\n记录的人员列表：\n";
	app.queue.push_marked(marked);
end

return {
	init = function(object, args)
		object.notes = args.notes or {};
		object.records = args.records or { { name = "milfoil-item_supervision_screen" } };
	end,
	actions = function(args)
		return {
			["core-read"] = function()
				local object_id = args.object.id;
				list_notes(object_id);
				list_objects(object_id);
			end,
			["core-skim"] = function()
				local object_id = args.object.id;
				list_objects(object_id);
			end,
		};
	end,
}
