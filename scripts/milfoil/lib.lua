local module = {};

require("app_aliases");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");
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

local lounge = {
	types = { "milfoil-staff_lounge", "milfoil-room" },
};

app.queue.push(obj.apply(lounge, "get_text", { field = "name" }));
app.actions:set("seat", app.i18n.t("milfoil-seat-name"), { {
	name = app.i18n.t("milfoil-seat-action-sit"),
	func = function()
		print("^_^");
	end
} });

return module;
