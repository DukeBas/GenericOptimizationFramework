import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Student implements Comparable<Student> {

	int id; // IDs are 0-based!
	ArrayList<Double> grades; // grades[i] contains grade for topic i
	Map<Integer, Integer> preferences;

	public Student(int ID) {
		id = ID;
		preferences = new HashMap<>();
		grades = new ArrayList<>();
	}

	public void addGrade(double grade) {
		grades.add(grade);
	}
    
	public double getGrade(int topic) {
        return grades.get(topic);
	}

    public void addPreference(int student, int preference) {
        preferences.put(student, preference);
    }

    public Integer getPref(Student student) {
        if (!preferences.containsKey(student.id)) return 5;
        else return preferences.get(student.id);
    }
    
    public Integer getPref(int studentID) {
        if (!preferences.containsKey(studentID)) return 5;
        else return preferences.get(studentID);
    }

	public boolean equals(Object o) {
		if (!(o instanceof Student))
			return false;
		return (id == ((Student) o).id);
	}

	public int compareTo(Student student) {
		return Integer.compare(id, student.id);
	}

}
