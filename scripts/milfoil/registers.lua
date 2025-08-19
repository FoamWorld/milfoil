local app = require("app");
local obj = require("objects");
obj.methods.name = function(object, args)
	for _, v in ipairs(object.types) do
		local tr = app.i18n._(v);
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
		local name = "";
		for _, v in ipairs(object.types) do
			-- looks like: milfoil-sit-on-milfoil-seat
			name = app.i18n._(key .. "-on-" .. v);
			if name ~= "" then
				break;
			end
		end
		if name == "" then
			name = app.i18n.t(key);
		end
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
