local app = require("app");
app.i18n.load_file("milfoil_ts.ftl");
app.i18n.load_file("plots.ftl");

local path = "milfoil/types/";
local obj = require("objects");
obj.declare("core-bar", require(path .. "bar"));
obj.declare("core-physical", require(path .. "physical"));
obj.declare("core-seat", require(path .. "seat"));

obj.declare("milfoil-item_supervision_screen", require(path .. "item_supervision_screen"))

obj.methods.name = function(object, args)
	for i = #object.types, 1, -1 do
		local type_name = object.types[i];
		local tr = app.i18n._("obj-" .. type_name);
		if tr ~= "" then
			return tr;
		end
	end
	return "???";
end;

obj.methods.setup_actions = function(object, args)
	local raw = {};
	for _, v in ipairs(object.types) do
		local type_data = obj.types[v];
		if type(type_data) == "table" then
			local actions = obj.exec(type_data.actions, args);
			if type(actions) == "table" then
				for key, value in pairs(actions) do
					raw[key] = raw[key] or value;
				end
			end
		end
	end

	local list = {};
	local ind = 1;
	for key, value in pairs(raw) do
		local name = app.i18n.t("obj-" .. key);
		list[ind] = {
			name = name,
			func = value,
		};
		ind = ind + 1;
	end

	app.actions:insert(object.id, obj.apply(object, "name"), list);
end;

obj.methods.get_text = function(object, args)
	local field = args.field;
	if type(object[field]) ~= "nil" then
		return object[field];
	end

	for i = #object.types, 1, -1 do
		local type_name = object.types[i];
		return app.i18n.t(type_name .. "-" .. field);
	end
end;

local tree = require("hierarchy");
tree.resolvers["milfoil"] = function(name)
	local func = require("milfoil/objects/" .. name);
	if type(func) ~= "function" then
		error("Target id \"" .. name .. "\" failed to resolve to a function.", 1);
	end
	return func();
end
