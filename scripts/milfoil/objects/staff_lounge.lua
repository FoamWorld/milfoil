local obj = require("objects");
local tree = require("hierarchy");

tree.templates:register("milfoil-staff_lounge", function(id)
	tree:add_child("#2", require("milfoil/objects/sofa"), id);
	tree:add_child("#3", require("milfoil/objects/staff_lounge_screen"), id);
end);

local staff_lounge = obj.new():with("milfoil-room");
table.insert(staff_lounge.types, "milfoil-staff_lounge");
staff_lounge.template = "milfoil-staff_lounge";

return staff_lounge;
