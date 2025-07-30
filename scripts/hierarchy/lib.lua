-- Use string for keys.
local indices = {
	parent = {},
	children = {},
};

Tree = {
	nodes = {
		root = {
			block = true,
		},
	},

	--[[
		0 = loaded;
		1 = remote-unique;
		2 = remote-clone;
	]]
	status = {},
};

function Tree:add_child(id, node, parent_id)
	parent_id = parent_id or "root";
	indices.parent[id] = parent_id;
	indices.children[parent_id] = {
		[id] = true
	};
	node.id = id;
	self.nodes[id] = node;
	self.status[id] = 0;
end

function Tree.add_child_schedule()
	-- todo
end

function Tree.setup_node()
	-- todo
end
