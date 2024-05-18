use crate::solution;
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};
use solution::{InstanceReader, LocalMove, Solution};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::Arc;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy)]
struct StudentIndex(usize);
#[derive(Clone, Debug, Copy)]
struct ProjectIndex(usize);
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy)]
struct MentorIndex(usize);

/// Each project has a person cap and a topic (one of the proficiencies)
#[derive(Clone, Debug)]
struct InputProject {
    person_cap: usize,
    topic: i32,
}

/// Each student is a hashmap of grades and set preferences for other students (usize)
#[derive(Debug)]
struct Student {
    grades: HashMap<i32, f64>,
    preferences: HashMap<StudentIndex, i32>,
}

/// Each mentor has a set of proficiencies
#[derive(Debug)]
struct Mentor {
    proficiencies: HashSet<i32>,
}

/// A project in the solution
#[derive(Clone, Debug)]
struct SolutionProject {
    project: InputProject,
    students: HashSet<StudentIndex>, // indexes into instance students
    mentor: MentorIndex,             // index into instance mentors
}

#[derive(Debug)]
pub struct AdHocInstance {
    dataset_name: String,
    projects: Vec<InputProject>,
    students: Vec<Student>,
    mentors: Vec<Mentor>,
    // Cost function parameters
    c_per: f64,
    c_coh: f64,
    c_work: f64,
    c_mstu: f64,
    c_mprof: f64,
    c_mnp: f64,
}

#[derive(Clone, Debug)]
pub struct AdHocSolution {
    instance: Arc<AdHocInstance>,
    projects: Vec<SolutionProject>,
    cost: f64,
    last_move: LastLocalMove,
    last_cost: f64,
}

#[derive(Clone, Debug)]
enum LastLocalMove {
    // Either a student swap with the project indexes and student indexes
    StudentSwap(
        ProjectIndex,
        StudentIndex,
        ProjectIndex,
        Option<StudentIndex>,
    ),
    // Or a mentor change with the old and new value
    MentorChange(ProjectIndex, MentorIndex, MentorIndex),
}

impl AdHocSolution {
    pub fn recompute_cost_from_scratch(&mut self) -> f64 {
        // Total cost is sum of perf cost, cohesion cost and mentor cost
        let perf_cost;
        let cohesion_cost;
        let mentor_cost;

        // Compute performance cost,
        // first get performance which is the average of the grade of the students in the project for that topic
        let mut performance = 0.0;
        for project in &self.projects {
            if project.students.len() == 0 {
                continue;
            }
            let mut project_cost = 0.0;
            for student in &project.students {
                // Get the grade of the student for the topic of the project
                project_cost += self.instance.students[student.0].grades[&project.project.topic];
            }
            performance += project_cost / project.students.len() as f64;
        }
        // Performance cost is 10 - (1/|non-empty groups|)*sum(average grade of students in group)
        let non_empty_groups = self
            .projects
            .iter()
            .filter(|p| p.students.len() > 0)
            .count();
        perf_cost = 10.0 - (performance / non_empty_groups as f64);
        // println!("perf: {} non_empty {}", perf_cost, non_empty_groups);

        // Compute cohesion cost
        // First compute cohesion, which is sum of cohesion per project
        let mut total_cohesion = 0.0;
        const DEFAULT_PREFERENCE: i32 = 5;
        for project in &self.projects {
            // Two cases, student alone or more than one student
            if project.students.len() == 1 {
                // Student preference for self
                let own_index = project.students.iter().next().unwrap().0;
                // Check preference for self, else use default value
                total_cohesion += *self
                    .instance
                    .students
                    .get(own_index)
                    .unwrap()
                    .preferences
                    .get(&StudentIndex(own_index))
                    .unwrap_or(&DEFAULT_PREFERENCE) as f64;
            } else if project.students.len() > 1 {
                let mut project_cohesion = 0.0;
                for student in &project.students {
                    let prefs = &self.instance.students[student.0].preferences;
                    for other_student in &project.students {
                        if student != other_student {
                            // Check if other student is in the preferences of the student
                            project_cohesion +=
                                *prefs.get(other_student).unwrap_or(&DEFAULT_PREFERENCE) as f64;
                        }
                    }
                }
                total_cohesion += project_cohesion / (project.students.len() - 1) as f64;
            }
        }

        // compute the cohesion cost, 10 - (1/number of students) * sum of cohesion per project
        cohesion_cost = 10.0 - (total_cohesion / self.instance.students.len() as f64);

        // Compute mentor cost
        // First computer for each mentor it's work cost, which is cMStu * (number of students supervised) + cMProf * (number of supervised projects with proficiency) + cMNP * (number of projects supervised with no proficiency)
        let mut mentor_costs = Vec::new();
        for (index, mentor) in self.instance.mentors.iter().enumerate() {
            let mentor_cost;
            let number_of_students = self
                .projects
                .iter()
                .filter(|p| p.mentor == MentorIndex(index))
                .map(|p| p.students.len())
                .sum::<usize>();
            let mut mentor_proficiency_projects = 0;
            let mut mentor_no_proficiency_projects = 0;
            for project in &self.projects {
                if project.mentor == MentorIndex(index) {
                    if mentor.proficiencies.contains(&project.project.topic) {
                        mentor_proficiency_projects += 1;
                    } else {
                        mentor_no_proficiency_projects += 1;
                    }
                }
            }
            mentor_cost = self.instance.c_mstu * number_of_students as f64
                + self.instance.c_mprof * mentor_proficiency_projects as f64
                + self.instance.c_mnp * mentor_no_proficiency_projects as f64;
            mentor_costs.push(mentor_cost);
        }
        // Mentor cost is 1/number of mentors * sum of mentor costs squared
        mentor_cost = (1.0 / self.instance.mentors.len() as f64)
            * mentor_costs.iter().map(|x| x.powi(2)).sum::<f64>();

        // Return final cost
        // println!(
        //     "perf: {}, coh: {}, mentor: {}",
        //     perf_cost, cohesion_cost, mentor_cost
        // );
        let final_cost = self.instance.c_per * perf_cost
            + self.instance.c_coh * cohesion_cost
            + self.instance.c_work * mentor_cost;
        println!(
            "final cost {}, perf= {:.2}x{:.2}, coh= {:.2}x{:.2}, mentor= {:.2}x{:.2}",
            final_cost,
            perf_cost,  self.instance.c_per,
            cohesion_cost,  self.instance.c_coh,
            mentor_cost , self.instance.c_work
        );
        self.cost = final_cost;
        final_cost
    }
}

impl Solution for AdHocSolution {
    fn get_cost(&mut self) -> f64 {
        self.cost
    }

    fn write_solution(&self, file_location: &str) {
        // File file = new File(filename);
        // try{
        //     PrintStream writer = new PrintStream(file);
        //     int filledGroups = groups.size();
        //     for (Group group : groups) {
        //         if (group.getGroupSize() == 0) filledGroups--;
        //     }

        //     writer.println(filledGroups);
        //     for (Group group : groups) {
        //     	if (group.getGroupSize() == 0) continue;
        //         writer.print(group.project.id + " " +  group.mentor.id + " " + group.students.size());
        //         for (Student student : group.students) {
        //             writer.print(" " + student.id);
        //         }
        //         writer.println();
        //     }
        //     writer.close();
        // } catch (IOException e) {
        //     System.out.println("Error: File not found.");
        //     e.printStackTrace();
        // }
        // Like code above but in Rust
        let file_path = format!(
            "{}/{}-{:.4}.out",
            file_location, self.instance.dataset_name, self.cost
        ); // todo make more robust
        let mut file =
            std::fs::File::create(file_path.clone()).expect("Could not save solution to file!!!");
        let mut filled_groups = 0;
        for project in &self.projects {
            if project.students.len() > 0 {
                filled_groups += 1;
            }
        }
        writeln!(file, "{}", filled_groups).expect("Could not write to file!!!");
        for (index, project) in self.projects.iter().enumerate() {
            if project.students.len() == 0 {
                continue;
            }
            write!(
                file,
                "{} {} {}",
                index,
                project.mentor.0,
                project.students.len()
            )
            .expect("Could not write to file!!!");
            for student in &project.students {
                write!(file, " {}", student.0).expect("Could not write to file!!!");
            }
            writeln!(file).expect("Could not write to file!!!");
        }
    }
}

pub struct AdHocMove;
impl LocalMove<AdHocSolution> for AdHocMove {
    fn do_random_move(solution: &mut AdHocSolution) {
        // Three local moves, swap students or mentors between groups
        let mut rng = thread_rng();
        let random_number =
            rng.gen_range(0..(solution.instance.students.len() + solution.instance.mentors.len()));

        // Check which move to do
        if random_number < solution.instance.students.len() {
            // Pick two projects, exchange student from one project with (possibly empty) student slot in another project
            let first_project = rng.gen_range(0..solution.projects.len());
            let second_project = rng.gen_range(0..solution.projects.len());

            // Continue if first project is empty
            if solution.projects[first_project].students.len() == 0
                || first_project == second_project
            {
                return;
            }

            // Pick a random student from first project
            let student_index = *solution.projects[first_project]
                .students
                .iter()
                .next()
                .unwrap();

            // Pick a random number for a slot in the second project
            let slot = rng.gen_range(0..solution.projects[second_project].project.person_cap);

            // Check if there is a student in that slot
            if slot < solution.projects[second_project].students.len() {
                // Swap students
                let student_index2 = *solution.projects[second_project]
                    .students
                    .iter()
                    .nth(slot)
                    .unwrap();
                solution.projects[second_project]
                    .students
                    .remove(&student_index2);
                solution.projects[second_project]
                    .students
                    .insert(student_index);
                solution.projects[first_project]
                    .students
                    .remove(&student_index);
                solution.projects[first_project]
                    .students
                    .insert(student_index2);

                // Update last move
                solution.last_move = LastLocalMove::StudentSwap(
                    ProjectIndex(first_project),
                    student_index,
                    ProjectIndex(second_project),
                    Some(student_index2),
                );

                // println!("Swapped students {} and {}", student_index.0, student_index2.0);
            } else {
                // Slot is empty, just move the student
                solution.projects[second_project]
                    .students
                    .insert(student_index);
                solution.projects[first_project]
                    .students
                    .remove(&student_index);

                // Update last move
                solution.last_move = LastLocalMove::StudentSwap(
                    ProjectIndex(first_project),
                    student_index,
                    ProjectIndex(second_project),
                    None,
                );

                // println!("Moved student {} to project {}", student_index.0, second_project);
            }
        } else {
            // Change mentor of a project to a random mentor
            let project = rng.gen_range(0..solution.projects.len());
            let new_mentor = MentorIndex(rng.gen_range(0..solution.instance.mentors.len()));
            let old_mentor = solution.projects[project].mentor;
            solution.projects[project].mentor = new_mentor;

            // Update last move
            solution.last_move =
                LastLocalMove::MentorChange(ProjectIndex(project), old_mentor, new_mentor);

            // println!("Changed mentor of project {} from {} to {}", project, old_mentor.0, new_mentor.0);
        }

        // Check if all groups are still under cap
        // for (index, project) in solution.projects.iter().enumerate() {
        //     if project.students.len() > project.project.person_cap {
        //         println!("{:?}", solution.projects);
        //         panic!(
        //             "Group {} is over cap len {} cap {}",
        //             index,
        //             project.students.len(),
        //             project.project.person_cap
        //         );
        //     }
        // }

        // Update last cost
        solution.last_cost = solution.cost;

        // Recompute cost
        solution.recompute_cost_from_scratch();
    }

    fn undo_last_move(solution: &mut AdHocSolution) {
        match solution.last_move {
            LastLocalMove::StudentSwap(
                first_project,
                student_index,
                second_project,
                student_index2,
            ) => {
                if let Some(student_index2) = student_index2 {
                    // Swap students
                    solution.projects[second_project.0]
                        .students
                        .remove(&student_index);
                    solution.projects[second_project.0]
                        .students
                        .insert(student_index2);
                    solution.projects[first_project.0]
                        .students
                        .remove(&student_index2);
                    solution.projects[first_project.0]
                        .students
                        .insert(student_index);
                } else {
                    // Slot is empty, just move the student
                    solution.projects[second_project.0]
                        .students
                        .remove(&student_index);
                    solution.projects[first_project.0]
                        .students
                        .insert(student_index);
                }
            }
            LastLocalMove::MentorChange(project, old_mentor, new_mentor) => {
                solution.projects[project.0].mentor = old_mentor;
            }
        }

        // Reset to last cost
        solution.cost = solution.last_cost;
    }
}

pub struct AdHocInstanceReader {}
impl InstanceReader<AdHocSolution> for AdHocInstanceReader {
    fn read_instance(&self, file_path: &str, instance_name: Option<&str>) -> AdHocSolution {
        // Get file
        let contents = std::fs::read_to_string(file_path).expect("Could not read file");
        let mut lines = contents.lines();

        // First line is s, p, m, t
        let mut first_line = lines.next().unwrap().split_whitespace();
        let s: usize = first_line.next().unwrap().parse().unwrap();
        let p: usize = first_line.next().unwrap().parse().unwrap();
        let m: usize = first_line.next().unwrap().parse().unwrap();
        let t: usize = first_line.next().unwrap().parse().unwrap();

        // Second line has cost constants
        let mut cost_constants = lines.next().unwrap().split_whitespace();
        let c_per: f64 = cost_constants.next().unwrap().parse().unwrap();
        let c_coh: f64 = cost_constants.next().unwrap().parse().unwrap();
        let c_work: f64 = cost_constants.next().unwrap().parse().unwrap();
        let c_mstu: f64 = cost_constants.next().unwrap().parse().unwrap();
        let c_mprof: f64 = cost_constants.next().unwrap().parse().unwrap();
        let c_mnp: f64 = cost_constants.next().unwrap().parse().unwrap();

        // Read the projects
        lines.next();
        let mut projects = Vec::new();
        for _ in 0..p {
            let mut project = lines.next().unwrap().split_whitespace();
            let person_cap: usize = project.next().unwrap().parse().unwrap();
            let topic: i32 = project.next().unwrap().parse().unwrap();
            projects.push(InputProject { person_cap, topic });
        }

        // Read the preferences
        let mut preferences = Vec::new();
        for _ in 0..s {
            lines.next();
            let mut student = lines.next().unwrap().split_whitespace();
            let mut stud_prefs = HashMap::new();
            let prefs: usize = student.next().unwrap().parse().unwrap();
            for _ in 0..prefs {
                let mut pref_line = lines.next().unwrap().split_whitespace();
                let student_pref: usize = pref_line.next().unwrap().parse().unwrap();
                let rj: i32 = pref_line.next().unwrap().parse().unwrap();
                stud_prefs.insert(StudentIndex(student_pref), rj);
            }
            preferences.push(stud_prefs);
        }

        // Read the grades
        lines.next();
        let mut grades = Vec::new();
        for _ in 0..s {
            let mut student = lines.next().unwrap().split_whitespace();
            let mut stud_grades = HashMap::new();
            for i in 0..t {
                let grade: f64 = student.next().unwrap().parse().unwrap();
                stud_grades.insert(i as i32, grade);
            }
            grades.push(stud_grades);
        }

        // Setup students
        let mut students = Vec::new();
        for i in 0..s {
            students.push(Student {
                grades: grades[i].clone(),
                preferences: preferences[i].clone(),
            });
        }

        // Read the mentors
        let mut mentors = Vec::new();
        lines.next();
        for _ in 0..m {
            let mut mentor = lines.next().unwrap().split_whitespace();
            let mut mentor_proficiencies = HashSet::new();
            let proficiencies: usize = mentor.next().unwrap().parse().unwrap();
            for _ in 0..proficiencies {
                // if read int is positive then it is a proficiency
                let proficiency: i32 = mentor.next().unwrap().parse().unwrap();
                if proficiency > 0 {
                    mentor_proficiencies.insert(proficiency);
                }
            }
            mentors.push(Mentor {
                proficiencies: mentor_proficiencies,
            });
        }

        // Initialize the solution where we randomly divide the students and mentors over the projects
        let mut solution_projects: Vec<SolutionProject> = Vec::new();
        for i in 0..p {
            solution_projects.push(SolutionProject {
                project: projects[i].clone(),
                students: HashSet::new(),
                mentor: MentorIndex(0),
            });
        }
        // Divide students over projects
        let mut rng = rand::thread_rng();
        for i in 0..s {
            // Take group cap into account
            let mut project_index = ProjectIndex(rng.gen_range(0..p));
            while solution_projects[project_index.0].students.len()
                >= solution_projects[project_index.0].project.person_cap
            {
                project_index = ProjectIndex(rng.gen_range(0..p));
            }

            // Assign student to project
            solution_projects[project_index.0]
                .students
                .insert(StudentIndex(i));
        }
        // Assign mentors to projects
        for i in 0..p {
            solution_projects[i].mentor = MentorIndex(rng.gen_range(0..m));
        }

        // Setup solution
        let mut solution = AdHocSolution {
            instance: Arc::new(AdHocInstance {
                dataset_name: instance_name.unwrap_or("unknown").to_string(),
                projects,
                students,
                mentors,
                c_per,
                c_coh,
                c_work,
                c_mstu,
                c_mprof,
                c_mnp,
            }),
            projects: solution_projects,
            cost: 0.0, // will get overriden by recompute_cost_from_scratch
            last_cost: 0.0,
            last_move: LastLocalMove::MentorChange(ProjectIndex(0), MentorIndex(0), MentorIndex(0)),
        };

        // println!("{:.?}", solution.projects);

        // Compute the cost of the initial solution
        solution.recompute_cost_from_scratch();

        // return
        solution
    }
}
