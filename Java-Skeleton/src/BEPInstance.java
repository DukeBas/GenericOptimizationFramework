import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

public class BEPInstance {
    
    final int S; // number of students
    final int P; // number of projects
    final int M; // number of mentors
    final int T; // numbers of topics

    // Cost function coefficients
    final double cPer;
    final double cCoh;
    final double cWork;
    final double cMStu;
    final double cMProf;
    final double cMNP;

    final List<Student> students; // IDs are 0-based, so student with ID i has index i in this list
    final List<Project> projects; // IDs are 0-based, so project with ID i has index i in this list
    final List<Mentor> mentors; // IDs are 0-based, so mentor with ID i has index i in this list

    double[][] gradeList; // gradeList[i][j] contains the grade of student with ID i for topic with ID j 
    boolean[][] proficiencyList; // proficiencyList[i][j] indicates if mentor with ID i is proficient in topic with ID j

    public BEPInstance(String filename) {
        int s = 0, p = 0, m = 0, t = 0;
        double tempcPer = 0, tempcCoh = 0, tempcWork = 0;
        double tempcMStu = 0, tempcMProf = 0, tempcMNP = 0;

        ArrayList<Project> projs = new ArrayList<>();
        ArrayList<Student> studs = new ArrayList<>();
        ArrayList<Mentor> ments = new ArrayList<>();

        File file = new File(filename);

        try {
            Scanner scanner = new Scanner(file);
            s = scanner.nextInt();
            p = scanner.nextInt();
            m = scanner.nextInt();
            t = scanner.nextInt();

            tempcPer = scanner.nextDouble();
            tempcCoh = scanner.nextDouble();
            tempcWork = scanner.nextDouble();
            tempcMStu = scanner.nextDouble();
            tempcMProf = scanner.nextDouble();
            tempcMNP = scanner.nextDouble();

            for (int i = 0; i < p; i++) { // read projs
                int ai = scanner.nextInt();
                int ti = scanner.nextInt();
                projs.add(new Project(i, ti, ai));
            }

            for (int i = 0; i < s; i++) { // read preferences
                Student student = new Student(i);
                int ni = scanner.nextInt();

                for (int j = 0; j < ni; j++) {
                    int sj = scanner.nextInt();
                    int rj = scanner.nextInt();
                    student.addPreference(sj, rj);
                }

                studs.add(student);	//The for loops over students can be combined to match new input (see problem description)
            }

            for (int i = 0; i < s; i++) { // read grades
                for (int j = 0; j < t; j++) { // read topics
                    studs.get(i).addGrade(scanner.nextDouble());
                }
            }

            for (int i = 0; i < m; i++) { // read ments
                Mentor mentor = new Mentor(i);

                int mi = scanner.nextInt();
                for (int j = 0; j < mi; j++) { // read proficiencies
                    int proficiency = scanner.nextInt();
                    mentor.proficiencies.add(proficiency);
                }

                ments.add(mentor);
            }

            scanner.close();
        } catch (FileNotFoundException e) {
			System.out.println("Could not read file!");
			e.printStackTrace();
		}

        S = s; P = p; M = m; T = t;
        cPer = tempcPer; cCoh = tempcCoh; cWork = tempcWork;
        cMStu = tempcMStu; cMProf = tempcMProf; cMNP = tempcMNP;

        students = Collections.unmodifiableList(studs);
        projects = Collections.unmodifiableList(projs);
        mentors = Collections.unmodifiableList(ments);

        gradeList = new double[S][T];
        proficiencyList = new boolean[M][T];

        for (int i = 0; i < S; i++) {
            for (int j = 0; j < T; j++) {
                gradeList[i][j] = studs.get(i).getGrade(j);
            }
        }

        for (int i = 0; i < M; i++) {
            for (int j = 0; j < T; j++) {
                proficiencyList[i][j] = ments.get(i).isProficient(j);
            }
        }
    }
}
