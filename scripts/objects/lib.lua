local module = {};

module.types = {};

module.funclist = {
	-- reserved: init
	exec = function(object, args)
		if type(object) == "function" or type(object) == "table" then
			return object(args);
		else
			return object;
		end
	end,
};

function module.apply(object, funcname, args)
	-- enable args.override?
	local property = object[funcname];
	if type(property) == "nil" then
		return module.funclist[funcname](object, args);
	else
		return module.apply(property, "exec", args);
	end
end

return module;
