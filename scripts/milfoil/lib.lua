local module = {};

require("app_aliases");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");

local lounge = {
	types = { "milfoil-staff_lounge", "milfoil-room" },
};

require("milfoil/registers");
local sofa = require("milfoil/objects/sofa");
obj.apply(sofa, "setup_actions");

return module;
