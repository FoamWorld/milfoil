local app = require("app");
local queue = app.queue;

local message_queue = {};
queue.flush = function()
	queue.push(message_queue);
	message_queue = {};
end

queue.push_plain = function(text)
	table.insert(message_queue, {
		text = text,
		mode = queue.MODE_PLAIN,
		level = 1
	});
end;

queue.push_marked = function(text)
	table.insert(message_queue, {
		text = text,
		mode = queue.MODE_MARKDOWN,
		level = 1
	});
end;
