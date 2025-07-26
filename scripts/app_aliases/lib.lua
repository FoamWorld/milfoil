local app = require("app");
local queue = app.queue;
queue.push = function (text)
	queue.push_message(text, app.queue.MODE_PLAIN, 1);
end;
queue.push_marked = function (text)
	queue.push_message(text, app.queue.MODE_MARKDOWN, 1);
end;
