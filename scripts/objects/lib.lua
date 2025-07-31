local module = {};

module.types = {};

module.funclist = {
	-- reserved: init
};

function module.apply(object, funcname, args)
	-- enable args.override?
	args = args or {};
	args.object = object;
	local property = object[funcname];
	if type(property) == "nil" then
		return module.funclist[funcname](object, args);
	else
		return module.exec(property, args);
	end
end

function module.call(object, funcname, args)
	for _, v in ipairs(object.types) do
		local type_data = module.types[v];
		if type_data ~= nil then
			if type_data[funcname] ~= "nil" then
				type_data[funcname](object, args);
			end
		end
	end
end

function module.clone(object)
	if type(object) ~= "table" then
		return object;
	end
	local table = {};
	for key, value in pairs(object) do
		table[module.clone(key)] = module.clone(value);
	end
	setmetatable(table, getmetatable(object))
	return table;
end

function module.exec(object, args)
	if type(object) == "function" or type(object) == "table" then
		return object(args);
	else
		return object;
	end
end

return module;
