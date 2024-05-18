import java.io.*;
import java.util.*;

public class BEPSolution {

    BEPInstance instance;
    ArrayList<Group> groups;
    double cost, prfCost, cohCost, loadCost; // cost values: correct after a call to getCost
    

    // does not create a valid solution (for efficient copying)
    public BEPSolution(BEPInstance inst) {
    	instance = inst;
    	groups = new ArrayList<Group>();
    }
    
    public BEPSolution(BEPInstance inst, boolean random) {

        instance = inst;
        
        // Make one group per project
        groups = new ArrayList<>();
        for (int i = 0; i < inst.P; i++) {
            Group g = new Group();
            g.setProject(inst.projects.get(i));
            groups.add(g);
        }

        // Add mentors to groups
        Random rand = new Random();
        if (random) {
            for (int i = 0; i < inst.P; i++) {
            	int k = rand.nextInt(inst.M);
                groups.get(i).setMentor(inst.mentors.get(k));     
            }
        } else {
            for (int i = 0; i < inst.P; i++) {
                groups.get(i).setMentor(inst.mentors.get(i % inst.M));
            }
        }

        // Add students to groups
        List<Integer> studIds = new ArrayList<>();
        for (int i = 0; i < inst.S; i++) studIds.add(i);
        if (random) Collections.shuffle(studIds);
        int j = 0;
        for (int i = 0; i < inst.S; i++) {
        	while (groups.get(j).getGroupSize() == groups.get(j).getProject().getCap()) j = (j + 1)%groups.size();
        	groups.get(j).addStudent(inst.students.get(studIds.get(i)));
            j = (j + 1)%groups.size();
        }

    }

    public BEPSolution copy() {
        BEPSolution sol = new BEPSolution(instance);
        for (Group G: groups) sol.groups.add(G.copy());
        return sol;
    }
    
    
    public boolean checkValid() {
        for (Group group : groups) {
        	if (group.getGroupSize() == 0) continue;
            if (group.students.size() > group.project.cap || group.mentor == null || group.project == null ||
                    group.project.id < 0 || group.project.id >= instance.P || group.mentor.id < 0 || group.mentor.id >= instance.M) {
                System.out.println("Basic Fail!");
            	return false;
            }
        }

        // Check that each student is used exactly once
        int[] studentsUsed = new int[instance.S];
        for (Group group : groups) {
            for (Student student : group.students) {
                studentsUsed[student.id]++;
            }
        }       
        for (int i = 0; i < instance.S; i++) {
        	if (studentsUsed[i] != 1) {
        		System.out.println("Student " + i + " is in " + studentsUsed[i] + " groups");
        		return false;
        	}
        }

        // Check that each project is used at most once
        int[] projectsUsed = new int[instance.P];
        for (Group group : groups) {
        	projectsUsed[group.project.id]++;
        }
        for (int i = 0; i < instance.P; i++) {
        	if (projectsUsed[i] > 1) {
        		System.out.println("Project " + i + " is done by more than 1 group");
        		return false;
        	}
        }
        
        return true;
    }

    // cost function, does not check validity!
    public double getCost() {
        
        prfCost = 0.0; cohCost = 0.0; loadCost = 0.0;
        int nGroups = 0;
        double[] mentorLoad = new double[instance.M];
        for (Group group : groups) {
        	if (group.getGroupSize() == 0) continue;
        	nGroups++;
        	prfCost += group.getGroupPerformance();
        	cohCost += group.getTotalCohesion();
            mentorLoad[group.mentor.id] += instance.cMStu * group.getGroupSize() + (instance.proficiencyList[group.mentor.id][group.project.topic] ? instance.cMProf : instance.cMNP);
        }

        for (int i = 0; i < instance.M; i++) loadCost += mentorLoad[i] * mentorLoad[i];
        
        if (nGroups != 0) prfCost = instance.cPer * (10.0 - prfCost / (double)nGroups);
        cohCost = instance.cCoh * (10.0 - cohCost / instance.S);
        loadCost *= instance.cWork / instance.M;
        cost = prfCost + cohCost + loadCost;
        
        return cost;
    }
    

	// -------------------------------- OUTPUT CODE ----------------------------------------------------
    public void visualize(String filename) {
        // No implementation :(
    }

    public void saveStats(String filename) {
        
        getCost();
        
        double[] mentorLoad = new double[instance.M];
        for (Group group : groups) {
        	if (group.getGroupSize() == 0) continue;
            mentorLoad[group.mentor.id] += instance.cMStu * group.getGroupSize() + (instance.proficiencyList[group.mentor.id][group.project.topic] ? instance.cMProf : instance.cMNP);
        }

        try {
            PrintWriter writer = new PrintWriter(filename);
            writer.println("Total Group performance cost: " + prfCost);
            writer.println("Total Group cohesion cost: " + cohCost);
            writer.println("Total Mentor workload cost: " + loadCost);
            writer.println("Total cost: " + cost);
            writer.println();

            
            for (Mentor ment: instance.mentors) {
                writer.println("---------------------------------------------------");
                writer.println("Mentor " + ment.id + " workload: " + mentorLoad[ment.id]);
            }
            writer.println();

            int gID = 1;
            for (Group group : groups) {
            	if (group.getGroupSize() == 0) continue;
                writer.println("---------------------------------------------------");
                writer.println("GROUP " + gID + ":");
                writer.println("Performance score: " + group.getGroupPerformance());
                writer.println("Total Cohesion score: " + group.getTotalCohesion());
                writer.println("Mentor of the group: " + group.mentor.id );
                writer.println("Is the mentor proficient: " + group.mentor.isProficient(group.project.topic));
                writer.println("Project of the group: " + group.project.id);
                writer.println("Size of the group: " + group.students.size() + " (max: " + group.project.cap + ")");
                writer.println("Students in the group: ");
                for (Student student : group.students) {
                    writer.print(student.id + " | ");
                    writer.print("Relevant grade: " + student.getGrade(group.project.topic));
                    writer.println();
                }
                writer.println("Preferences of the students:");
                writer.println("Each line contains the preferences of a student for all other students in the group in the form (id_other_student, preference)");
                for (Student student : group.students){
                    writer.print("Student " + student.id + ": ");
                    for (Student other : group.students){
                        if (student.id != other.id){
                            writer.print("(" + other.id + "," + student.getPref(other) + ") ");
                        }
                    }
                    writer.println();
                }
            writer.println();
            gID++;
        }
            writer.close();
        } catch (FileNotFoundException e) {
            System.out.println("Error: File not found.");
            e.printStackTrace();
        }
    }

    public void output(String filename) {
        File file = new File(filename);
        try{
            PrintStream writer = new PrintStream(file);
            int filledGroups = groups.size();
            for (Group group : groups) {
                if (group.getGroupSize() == 0) filledGroups--;
            }

            writer.println(filledGroups);
            for (Group group : groups) {
            	if (group.getGroupSize() == 0) continue;
                writer.print(group.project.id + " " +  group.mentor.id + " " + group.students.size());
                for (Student student : group.students) {
                    writer.print(" " + student.id);
                }
                writer.println();
            }
            writer.close();
        } catch (IOException e) {
            System.out.println("Error: File not found.");
            e.printStackTrace();
        }
    }
}
