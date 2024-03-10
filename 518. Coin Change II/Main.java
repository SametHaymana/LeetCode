import java.util.*;

class Node {
    int val;
    Map<Node, Integer> neighbors; // Neighbors and their respective weights

    Node(int val) {
        this.val = val;
        this.neighbors = new HashMap<>();
    }

    void addNeighbor(Node neighbor, int weight) {
        neighbors.put(neighbor, weight);
    }
}

class Graph {
    List<Node> nodes;

    Graph() {
        nodes = new ArrayList<>();
    }

    void addNode(Node node) {
        nodes.add(node);
    }

    public int countPathsBFS(Node start, int targetWeight) {
        int pathCount = 0;
        Queue<Path> queue = new LinkedList<>();

        // Start BFS from the start node
        queue.add(new Path(start, 0));

        while (!queue.isEmpty()) {
            Path currentPath = queue.poll();
            Node currentNode = currentPath.lastNode;
            int currentWeight = currentPath.totalWeight;

            // Check if the current weight equals the target weight
            if (currentWeight == targetWeight) {
                pathCount++;
            }

            // Add neighbors to the queue
            for (Map.Entry<Node, Integer> entry : currentNode.neighbors.entrySet()) {
                Node neighbor = entry.getKey();
                int edgeWeight = entry.getValue();

                if (currentWeight + edgeWeight <= targetWeight) {
                    queue.add(new Path(neighbor, currentWeight + edgeWeight));
                }
            }
        }

        return pathCount;
    }

    static class Path {
        Node lastNode;
        int totalWeight;

        Path(Node lastNode, int totalWeight) {
            this.lastNode = lastNode;
            this.totalWeight = totalWeight;
        }
    }
}



class Solution {
    public int change(int amount, int[] coins) {
       // Create graph
        Graph graph = new Graph();

        // Create nodes
        Node[] nodes = new Node[amount + 1];
        for (int i = 0; i <= amount; i++) {
            nodes[i] = new Node(i);
            graph.addNode(nodes[i]);
        }

        // Create edges
        for (int i = 0; i < coins.length; i++) {
            for (int j = 1; j <= amount; j++) {
                if (j - coins[i] >= 0) {
                    nodes[j].addNeighbor(nodes[j - coins[i]], coins[i]);
                }
            }
        }

        // Count paths
        return graph.countPathsBFS(nodes[0], amount);
    }
}



public class Main {

    public static void main(String[] args) {
        // write your code here
        Solution sol = new Solution();

        int[] coins = {1, 2,3};

        System.out.println(sol.change(4, coins));
    }
}