import java.util.Locale;

public class BEPAssignment {
    public static void main(String[] args) {
        Locale.setDefault(new Locale("en", "US"));
        
		String dataset = "small"; // choose the dataset
		BEPInstance inst = new BEPInstance("data/" + dataset + ".txt"); // load the problem instance
		BEPSolution sol = new BEPSolution(inst, true); // initialize a (random) solution
		
		// TODO: compute good solution
		
		if (!sol.checkValid()) System.out.println("Solution not valid!"); // check validity of solution
		System.out.println("Cost = " + sol.getCost()); // output the cost
		System.out.println("prfCost = " + sol.prfCost + ", cohCost = " + sol.cohCost + ", loadCost = " + sol.loadCost); // output the subcosts
		sol.output("output/" + dataset + ".out"); // output the solution
		sol.saveStats("stats/" + dataset + ".txt"); // save the solution stats
    }
    
}
