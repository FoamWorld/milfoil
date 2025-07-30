local app = require("app");
local obj = require("objects");
obj.funclist.name = function(object, args)
	for _, v in ipairs(object.types) do
		local tr = app.i18n._(v);
		if tr ~= "" then
			return tr;
		end
	end
	return "???";
end;

obj.funclist.setup_actions = function(object, args)
	app.actions:set("{{ id }}", obj.apply(object, "name"), { {
		name = app.i18n.t("milfoil-seat-action-sit"),
		func = function()
			print("^_^");
		end
	} });
end;

obj.funclist.get_text = function(object, args)
	local field = args.field;
	if type(object[field]) ~= "nil" then
		return object[field];
	end
	for _, v in ipairs(object.types) do
		local key = v .. "-" .. field;
		return app.i18n.t(key);
	end
end;
