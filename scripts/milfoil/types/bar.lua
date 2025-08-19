return {
	init = function(object, args)
		local max = args.max or math.huge;
		local value = args.value or max;
		local speed = args.speed or 0.0;
		object.max = max;
		object.value = value;
		object.speed = speed;
	end,
	mutate = function(object, delta)
		object.value = object.value + delta;
	end,
	mutate_speed = function(object, delta)
		object.speed = object.speed + delta;
	end
}
