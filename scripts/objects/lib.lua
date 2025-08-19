local module = {};

local function with_type(self, type_name, args)
	table.insert(self.types, type_name);

	local type_data = module.types[type_name];
	if type(type_data) == "table" then
		local initializer = module.types[type_name].init;
		if type(initializer) == "function" then
			initializer(self, args);
		end
	end

	return self;
end

module.types = {};

module.methods = {};

function module.declare(type_name, args)
	module.types[type_name] = args;
end

function module.new()
	return { types = {}, with = with_type };
end

function module.apply(object, funcname, args)
	-- enable args.override?
	args = args or {};
	args.object = object;
	local property = object[funcname];
	if type(property) == "nil" then
		return module.methods[funcname](object, args);
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
