local tree = {
	-- 节点的数据
	nodes = {
		root = { types = {} },
	},

	-- 索引信息，使用字符串。
	parent = {
		root = "root",
	},
	children = {
		root = {},
	},

	-- 目标节点的状态
	status = {},

	NODE_LOADED = 0, -- 存在于 `Tree` 中
	NODE_INSAVE = 1, -- 存在于当前存档中
	NODE_UNIQUE = 2, -- 待生成，对应的数据不得被以 `NODE_CLONE` 形式引用
	NODE_CLONE = 3, -- 待生成，会复制一份
	NODE_CUSTOM = 4, -- 待生成，生成方式自定义
	NODE_OTHERS = 5, -- 其它状态
};

function tree:add_child(id, node, parent_id)
	parent_id = parent_id or "root";
	self.parent[id] = parent_id;
	self.children[id] = {};
	self.children[parent_id][id] = true;

	node.id = id;
	self.nodes[id] = node;
	self.status[id] = tree.NODE_LOADED;
end

function tree:get_children_id(parent_id)
	-- set
	return self.children[parent_id];
end

function tree:get_parent_id(id)
	return self.parent[id];
end

function tree:get_grandparent_id(id)
	return self.parent[self.parent[id]];
end

--[[ 与生成有关的构件 ]]

Templates = {};

local obj = require("objects");

-- 门径。与单个指定节点连接。
obj.types["portal"] = {
	init = function(object, args)
		object.open = args.open or false;
		object.lock = args.lock;
		object.target_id = args.target_id;
		object.target_type = args.target_type or tree.NODE_CLONE;
	end
};

-- 懒加载标记。
obj.types["lazy"] = {
	init = function(object, args)
		object.is_lazy = true;
	end
};

-- 拼图。生成时行为：生成并与新节点的随机拼图对应，然后填充 `portal` 的相关值。
obj.types["jigsaw"] = {
	init = function(object, args)
		object.is_jigsaw = true;
		object.relative_id = args.id;
		object.target_id = args.target_id;
		object.target_type = args.target_type or tree.NODE_CLONE;
		-- object.filter?

		local parent = tree.nodes[tree:get_parent_id(object.id)];
		table.insert(parent.children_id, args.id);
	end,
	gen = function(object)
		local parent_id = tree:get_grandparent_id(object.id);
		local target_id = tree:reach(parent_id, object.target_id, object.target_type);

		local target_jigsaw_list = {};
		for _, child_id in ipairs(tree:get_children_id(target_id)) do
			local child = tree.nodes[child_id];
			if child.is_jigsaw then
				table.insert(target_jigsaw_list, child_id);
			end
		end

		if #target_jigsaw_list == 0 then
			print("Warning: cannot link jigsaw " .. object.id .. " with " .. target_id);
			return;
		end

		local selected_jigsaw_id = math.random(#target_jigsaw_list);

		object.target_id = selected_jigsaw_id;
		object.target_type = tree.NODE_LOADED;
	end
};

function tree:reach(parent_id, target_id, target_type)
	-- todo
end

return tree;
