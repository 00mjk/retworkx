retworkx API
=============

.. py:class:: PyDAG
   A class for creating direct acyclic graphs.

   The PyDAG class is constructed using the Rust library `daggy`_ which is
   itself built on the Rust library `petgraph`_. The limitations and quirks
   with both libraries dictate how this operates. The biggest thing to be
   aware of when using the PyDAG class is that while node and edge indexes
   are used for accessing elements on the DAG, node removal can change the
   index of a node `petgraph`_. The limitations and quirks
   with both libraries dictate how this operates. The biggest thing to be
   aware of when using the PyDAG class is that while node and edge indexes
   are used for accessing elements on the DAG, node removal can change the
   indexes of nodes. Basically when a node in the middle of the dag is
   removed the last index is moved to fill that spot. This means either
   you have to track that event, or on node removal update the indexes for
   the nodes you care about.

    .. py:method:: __init__(self):
        Initialize an empty DAG.

    .. py:method:: edges(self):
        Return a list of all edge data.

        :returns: A list of all the edge data objects in the DAG
        :rtype: list

    .. py:method:: nodes(self):
        Return a list of all node data.

        :returns: A list of all the node data objects in the DAG
        :rtype: list

    .. py:method:: successors(self, node):
        Return a list of all the node successor data.

        :param int node: The index for the node to get the successors for

        :returns: A list of the node data for all the child neighbor nodes
        :rtype: list

    .. py:method:: predecessors(self, node):
        Return a list of all the node predecessor data.

        :param int node: The index for the node to get the predecessors for

        :returns: A list of the node data for all the parent neighbor nodes
        :rtype: list

    .. py:method:: get_edge_data(self, node_a, node_b):
        Return the edge data for the edge between 2 nodes.

        :param int node_a: The index for the first node
        :param int node_b: The index for the second node

        :returns: The data object set for the edge
        :raises: When there is no edge between nodes

    .. py:method:: remove_node(self, node):
        Remove a node from the DAG.

        NOTE: Removal of a node may change the index for other nodes in the
        DAG. The last node will shift to the index of the removed node to take
        its place.

        :param int node: The index of the node to remove

    .. py:method:: add_edge(self, parent, child, edge):
        Add an edge between 2 nodes.

        Use add_child() or add_parent() to create a node with an edge at the
        same time as an edge for better performance.

        :param int parent: Index of the parent node
        :param int child: Index of the child node
        :param edge: The object to set as the data for the edge. It can be any
            python object.

        :raises: When the new edge will create a cycle

    .. py:method:: add_node(self, obj):
        Add a new node to the dag.

        :param obj: The python object to attach to the node

        :returns index: The index of the newly created node
        :rtype: int

    .. py:method:: add_child(self, parent, obj, edge):
        Add a new child node to the dag.

        This will create a new node on the dag and add an edge from the parent
        to that new node.

        :param int parent: The index for the parent node
        :param obj: The python object to attach to the node
        :param edge: The python object to attach to the edge

        :returns index: The index of the newly created child node
        :rtype: int

    .. py:method:: add_parent(self, child, obj, edge):
        Add a new parent node to the dag.

        This create a new node on the dag and add an edge to the child from
        that new node

        :param int child: The index of the child node
        :param obj: The python object to attach to the node
        :param edge: The python object to attach to the edge

        :returns index: The index of the newly created parent node
        :rtype: int

    .. py:method:: adj(self, node):
        Get the index and data for the neighbors of a node.

        This will return a dictionary of node indexes that contains the data
        objects for the nodes which have an edge (inbound or outbound) with
        the specified node.

        :param int node: The index of the node to get the neighbors

        :returns neighbors: A dictionary where the keys are node indexes and
            the value is the node data object for all nodes that share an
            edge with the specified node.
        :rtype: dict

    .. py:method:: adj_direction(self, node, direction):
        Get the index and data for either the parent or children of a node.

        This will return a dictionary of node indexes that contains the data
        objects for the nodes which have an edge (inbound or outbound as
        specified) with the specified node.

        :param int node: The index of the node to get the neighbors
        :param bool direction: The direction to use for finding nodes,
            True means inbound edges and False means outbound edges.

        :returns neighbors: A dictionary where the keys are node indexes and
            the value is the node data object for all nodes that share an
            edge with the specified node.
        :rtype: dict

    .. py:method:: in_degree(self, node):
        Get the degree of a node for inbound edges.

        :param int node: The index of the node to find the inbound degree of

        :returns degree: The inbound degree for the specified node
        :rtype: int

.. _daggy: https://github.com/mitchmindtree/daggy
.. _petgraph: https://github.com/bluss/petgraph

.. py:function:: dag_longest_path_length(graph):
    Find the length of the longest path in a graph.

    :param PyDAG graph: The graph to find the longest path on

    :returns length: The longest path length on the graph
    :rtype: int

.. py:function:: number_weakly_connected_components(graph):
    Find the number of weakly connected components in a DAG.

    :param PyDAG graph: The graph to find the number of weakly connected
        components on

    :returns number: The number of weakly connected components in the DAG
    :rtype: int

.. py:function:: is_isomorphic(first, second):
    Determine if 2 DAGS are structurally isomorphic.

    This checks igf 2 graphs are structurally isomorphic (it doesn't match
    the contents of the nodes or edges on the dag).

    :param PyDAG first: The first DAG to compare
    :param PyDAG second: The second DAG to compare

    :returns is_isomorphic: True if the 2 PyDAGs are structurally isomorphic
        False if they are not.
    :rtype: bool
