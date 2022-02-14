use std::collections::{HashMap, HashSet};

use crate::Task;

pub trait Visualizable {
    fn visualize(&self, visualizer: &mut impl Visualizer);
}

pub trait Visualizer {
    fn task(&mut self, task: *const Task, name: &str, state: &str) -> bool;
    // fn node(&mut self, node: *const Node, type_name: &str, state: &str) -> bool;
    // fn input(&mut self, task: *const Task, node: *const Node);
    // fn output(&mut self, task: *const Task, node: *const Node);
    fn children_start(&mut self, parent_task: *const Task);
    fn child(&mut self, parent_task: *const Task, child_task: *const Task);
    fn children_end(&mut self, parent_task: *const Task);
    // fn nested_start(&mut self, parent_node: *const Node);
    // fn nested(&mut self, parent_node: *const Node, nested_node: *const Node);
    // fn nested_end(&mut self, parent_node: *const Node);
    // fn dependency(&mut self, task: *const Task, node: *const Node);
    // fn created(&mut self, task: *const Task, node: *const Node);
}

// pub struct GraphViz {
//     include_nodes: bool,
//     visited: HashSet<usize>,
//     output_has_task: HashSet<usize>,
//     id_map: HashMap<usize, usize>,
//     output: String,
//     edges: Vec<(usize, usize, String)>,
// }

// impl GraphViz {
//     pub fn new(include_nodes: bool) -> Self {
//         Self {
//             include_nodes,
//             visited: HashSet::new(),
//             output_has_task: HashSet::new(),
//             id_map: HashMap::new(),
//             output: String::new(),
//             edges: Vec::new(),
//         }
//     }

//     fn get_id<T>(&mut self, ptr: *const T) -> usize {
//         let ptr = ptr as usize;
//         match self.id_map.get(&ptr) {
//             Some(id) => *id,
//             None => {
//                 let id = self.id_map.len() + 1;
//                 self.id_map.insert(ptr, id);
//                 id
//             }
//         }
//     }

//     pub fn wrap_html(graph: &str) -> String {
//         format!("<!DOCTYPE html>
//           <html>
//           <head>
//             <meta charset=\"utf-8\">
//             <title>Graph</title>
//           </head>
//           <body>
//             <script src=\"https://cdn.jsdelivr.net/npm/viz.js@2.1.2-pre.1/viz.js\"></script>
//             <script src=\"https://cdn.jsdelivr.net/npm/viz.js@2.1.2-pre.1/full.render.js\"></script>
//             <script>
//               const s = `{}`;
//               new Viz().renderSVGElement(s).then(el => document.body.appendChild(el)).catch(e => console.error(e));
//             </script>
//           </body>
//           </html>", escape(graph))
//     }
// }

// impl ToString for GraphViz {
//     fn to_string(&self) -> String {
//         return "digraph {
//                   rankdir=LR
//                   "
//         .to_string()
//             + &self.output
//             + &self
//                 .edges
//                 .iter()
//                 .filter(|(a, b, _)| self.visited.contains(a) && self.visited.contains(b))
//                 .map(|(_, _, o)| o.as_str())
//                 .collect::<String>()
//             + "}";
//     }
// }

// fn escape(s: &str) -> String {
//     s.replace("\\", "\\\\")
//         .replace("\"", "\\\"")
//         .replace("\n", "\\n")
// }

// impl Visualizer for GraphViz {
//     fn task(&mut self, task: *const Task, name: &str, state: &str) -> bool {
//         let id = self.get_id(task);
//         if self.visited.contains(&id) {
//             false
//         } else {
//             self.visited.insert(id);
//             self.output += &format!(
//                 "{} [shape=box, label=\"{}\"]\n",
//                 id,
//                 escape(&if state == "" {
//                     name.to_string()
//                 } else {
//                     name.to_string() + "\n" + state
//                 })
//             );
//             true
//         }
//     }

//     fn node(&mut self, node: *const Node, type_name: &str, state: &str) -> bool {
//         if !self.include_nodes && state == "" {
//             return true;
//         }
//         let id = self.get_id(node);
//         if self.visited.contains(&id) {
//             false
//         } else {
//             self.visited.insert(id);
//             self.output += &format!(
//                 "{} [label=\"{}\"]\n",
//                 id,
//                 escape(&if state == "" {
//                     type_name.to_string()
//                 } else {
//                     type_name.to_string() + "\n" + state
//                 })
//             );
//             true
//         }
//     }

//     fn output(&mut self, task: *const Task, node: *const Node) {
//         let task = self.get_id(task);
//         let node = self.get_id(node);
//         if !self.output_has_task.contains(&node) {
//             self.output_has_task.insert(node);
//             // self.edges += &format!("{}:e -> {}:w [color=red]\n", task, node);
//             if self.visited.contains(&node) && self.visited.contains(&task) {
//                 self.output += &format!(
//                     "subgraph cluster_{} {{\ncolor=lightgray; {}:e -> {}:w [color=red]\n}}\n",
//                     node, task, node
//                 );
//             }
//         } else {
//             self.edges.push((
//                 task,
//                 node,
//                 format!(
//                     "{}:e -> {}:n [color=\"#990000\", constraint=false]\n",
//                     task, node
//                 ),
//             ));
//         }
//     }

//     fn input(&mut self, task: *const Task, node: *const Node) {
//         let task = self.get_id(task);
//         let node = self.get_id(node);
//         if !self.visited.contains(&task) || !self.visited.contains(&node) {
//             return;
//         }
//         self.edges.push((
//             node,
//             task,
//             format!("{} -> {} [color=\"#009129\"]\n", node, task),
//         ));
//     }

//     fn dependency(&mut self, task: *const Task, node: *const Node) {
//         let task = self.get_id(task);
//         let node = self.get_id(node);
//         self.edges.push((
//             node,
//             task,
//             if self.include_nodes {
//                 format!(
//                     "{} -> {} [style=dotted, weight=0, arrowhead=empty, color=gray, constraint=false]\n",
//                     node, task
//                 )
//             } else {
//                 format!(
//                     "{} -> {} [style=dashed, weight=0, arrowhead=empty, color=\"#009129\", constraint=false]\n",
//                     node, task
//                 )
//             }
//         ));
//     }

//     fn created(&mut self, task: *const Task, node: *const Node) {
//         let task = self.get_id(task);
//         let node = self.get_id(node);
//         self.edges.push((
//             task,
//             node,
//             if self.include_nodes {
//                 format!(
//                     "{} -> {} [weight=0, arrowhead=empty, color=gray, constraint=false]\n",
//                     task, node
//                 )
//             } else {
//                 format!(
//                     "{} -> {} [style=dashed, weight=0, arrowhead=empty, color=red, constraint=false]\n",
//                     task, node
//                 )
//             },
//         ));
//     }

//     fn children_start(&mut self, parent_task: *const Task) {
//         let parent_task = self.get_id(parent_task);
//         self.output += &format!("subgraph cluster_{} {{\nrank=same\n", parent_task);
//     }

//     fn child(&mut self, parent_task: *const Task, child_task: *const Task) {
//         let parent_task = self.get_id(parent_task);
//         let child_task = self.get_id(child_task);
//         if self.visited.contains(&parent_task) && self.visited.contains(&child_task) {
//             self.output += &format!(
//                 "{}:e -> {}:w [style=dashed, color=lightgray]\n",
//                 parent_task, child_task
//             );
//         }
//     }

//     fn children_end(&mut self, _parent_task: *const Task) {
//         self.output += &format!("}}\n");
//     }

//     fn nested_start(&mut self, parent_node: *const Node) {
//         if !self.include_nodes {
//             return;
//         }
//         let parent_node = self.get_id(parent_node);
//         self.output += &format!("subgraph cluster_{} {{\ncolor=\"#c2e4ff\"\n", parent_node);
//     }

//     fn nested(&mut self, parent_node: *const Node, nested_node: *const Node) {
//         let parent_node = self.get_id(parent_node);
//         let nested_node = self.get_id(nested_node);
//         if self.visited.contains(&parent_node) && self.visited.contains(&nested_node) {
//             self.output += &format!("{} -> {} [color=\"#94c8f2\"]\n", parent_node, nested_node);
//         }
//     }

//     fn nested_end(&mut self, _parent_node: *const Node) {
//         if !self.include_nodes {
//             return;
//         }
//         self.output += &format!("}}\n");
//     }
// }
