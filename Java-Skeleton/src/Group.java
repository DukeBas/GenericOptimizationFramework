import java.util.ArrayList;

public class Group {
	
	ArrayList<Student> students;
	Mentor mentor;
	Project project;

	public Group() {
		students = new ArrayList<Student>();
		mentor = null;
		project = null;
	}
	
	public Group copy() {
		Group G = new Group();
		G.mentor = mentor;
		G.project = project;
		G.students.addAll(students);
		return G;
	}

	public void setProject(Project project) {
		this.project = project;
	}

	public void setMentor(Mentor mentor) {
		this.mentor = mentor;
	}

	public void addStudent(Student student) {
		this.students.add(student);
	}

	public Project getProject() {
		return this.project;
	}
	
	public Mentor getMentor() {
		return this.mentor;
	}
	
	public int getGroupSize() {
		return students.size();
	}

	public double getTotalCohesion() {
		if (this.getGroupSize() == 0) {
			return 0;
		}
		if (this.getGroupSize() == 1) {
			Student s = students.get(0);
			return s.getPref(s.id);
		}
		double total_cohesion = 0;
		for (int i = 0; i < students.size(); i++) {
			double student_cohesion = 0.0;
			for (int j = 0; j < students.size(); j++) {
				if (i == j) continue;
				student_cohesion += students.get(i).getPref(students.get(j));
			}
			total_cohesion += student_cohesion / (students.size() - 1.0);
		}
		return total_cohesion;
	}

	public double getGroupPerformance() {
		if (this.getGroupSize() == 0) {
			return 0;
		}
		double total = 0;
		int group_topic = this.project.topic;

		for (Student s : students) {
			total += s.getGrade(group_topic);
		}

		return total/students.size();
	}

	public boolean containsStudentID(int ID) {
		for (Student s : students)
			if (s.id == ID)
				return true;
		return false;
	}

	public boolean containsStudent(Student s) {
		return students.contains(s);
	}

	public void removeStudent(Student s) {
		students.remove(s);
	}

	public void clear() {
		students.clear();
	}

}
