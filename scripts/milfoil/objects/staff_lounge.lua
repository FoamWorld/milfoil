local obj = require("objects");
local tree = require("hierarchy");

tree.templates:register("milfoil-staff_lounge", function(id)
	local sofa = obj.new():with("core-physical", { size = 2.0 }):with("core-seat");
	tree:add_child(tree.idgen:combined("sofa"), sofa, id);

	local screen = obj.new():with("core-terminal"):with("core-screen"):with("milfoil-item_supervision_screen");
	tree:add_child(tree.idgen:combined("screen"), screen, id);
end);

local function build()
	local lounge = obj.new():with("core-room");
	table.insert(lounge.types, "milfoil-staff_lounge");
	lounge.template = "milfoil-staff_lounge";
	return lounge;
end

return build;
