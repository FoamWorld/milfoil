return {
	new = function(max, speed)
		speed = speed or 0.0
		return {
			types = { "bar" },
			max = max,
			value = max,
			speed = speed,
		}
	end,
	mutate = function(object, delta)
		object.value = object.value + delta;
	end,
	mutate_speed = function(object, delta)
		object.speed = object.speed + delta;
	end
}
