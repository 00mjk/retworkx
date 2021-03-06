# Licensed under the Apache License, Version 2.0 (the "License"); you may
# not use this file except in compliance with the License. You may obtain
# a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
# WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
# License for the specific language governing permissions and limitations
# under the License.

import unittest
import retworkx


class TestUnion(unittest.TestCase):
    def test_union_merge_all(self):
        dag_a = retworkx.PyDiGraph()
        dag_b = retworkx.PyDiGraph()

        node_a = dag_a.add_node("a_1")
        dag_a.add_child(node_a, "a_2", "e_1")
        dag_a.add_child(node_a, "a_3", "e_2")

        node_b = dag_b.add_node("a_1")
        dag_b.add_child(node_b, "a_2", "e_1")
        dag_b.add_child(node_b, "a_3", "e_2")

        dag_c = retworkx.digraph_union(dag_a, dag_b, True, True)

        self.assertTrue(retworkx.is_isomorphic(dag_a, dag_c))

    def test_union_basic_merge_edges_only(self):
        dag_a = retworkx.PyDiGraph()
        dag_b = retworkx.PyDiGraph()

        node_a = dag_a.add_node("a_1")
        dag_a.add_child(node_a, "a_2", "e_1")
        dag_a.add_child(node_a, "a_3", "e_2")

        node_b = dag_b.add_node("a_1")
        dag_b.add_child(node_b, "a_2", "e_1")
        dag_b.add_child(node_b, "a_3", "e_2")

        dag_c = retworkx.digraph_union(dag_a, dag_b, False, True)

        self.assertTrue(len(dag_c.edge_list()) == 2)
        self.assertTrue(len(dag_c.nodes()) == 6)

    def test_union_basic_merge_nodes_only(self):
        dag_a = retworkx.PyDiGraph()
        dag_b = retworkx.PyDiGraph()

        node_a = dag_a.add_node("a_1")
        child_a = dag_a.add_child(node_a, "a_2", "e_1")
        dag_a.add_child(node_a, "a_3", "e_2")

        node_b = dag_b.add_node("a_1")
        dag_b.add_child(node_b, "a_2", "e_1")
        dag_b.add_child(node_b, "a_3", "e_2")

        dag_c = retworkx.digraph_union(dag_a, dag_b, True, False)

        self.assertTrue(len(dag_c.edge_list()) == 4)
        self.assertTrue(len(dag_c.get_all_edge_data(node_a, child_a)) == 2)
        self.assertTrue(len(dag_c.nodes()) == 3)

    def test_union_basic_merge_none(self):
        dag_a = retworkx.PyDiGraph()
        dag_b = retworkx.PyDiGraph()

        node_a = dag_a.add_node("a_1")
        dag_a.add_child(node_a, "a_2", "e_1")
        dag_a.add_child(node_a, "a_3", "r_2")

        node_b = dag_b.add_node("a_1")
        dag_b.add_child(node_b, "a_2", "e_1")
        dag_b.add_child(node_b, "a_3", "e_2")

        dag_c = retworkx.digraph_union(dag_a, dag_b, False, False)

        self.assertTrue(len(dag_c.nodes()) == 6)
        self.assertTrue(len(dag_c.edge_list()) == 4)
