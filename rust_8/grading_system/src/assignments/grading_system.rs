// Grading System
pub fn grading_system() {
    let students: Vec<Student> = vec![
        create_student("David".to_string(), 96),
        create_student("Jemila".to_string(), 36),
    ];

    for student in students {
        println!(
            "{} scored {} and his grade is: {:?} (GPA: {:?}.)",
            student.name,
            student.score,
            student.grade,
            calculate_gpa(&student.grade)
        );
    }
}

#[derive(Debug)]
enum LetterGrade {
    APlus,
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    DMinus,
    F,
}

#[derive(Debug)]
struct Student {
    name: String,
    score: u8,
    grade: LetterGrade,
}

fn create_student(name: String, score: u8) -> Student {
    Student {
        name,
        score,
        grade: calculate_grade(score),
    }
}

fn calculate_grade(score: u8) -> LetterGrade {
    match score {
        97..=100 => LetterGrade::APlus,
        93..=96 => LetterGrade::A,
        90..=92 => LetterGrade::AMinus,
        87..=89 => LetterGrade::BPlus,
        83..=86 => LetterGrade::B,
        80..=82 => LetterGrade::BMinus,
        77..=79 => LetterGrade::CPlus,
        73..=76 => LetterGrade::C,
        70..=72 => LetterGrade::CMinus,
        67..=69 => LetterGrade::DPlus,
        63..=66 => LetterGrade::D,
        60..=62 => LetterGrade::DMinus,
        _ => LetterGrade::F,
    }
}

// Calculate GPA
fn calculate_gpa(grade: &LetterGrade) -> f32 {
    match grade {
        LetterGrade::APlus => 4.0,
        LetterGrade::A => 3.9,
        LetterGrade::AMinus => 3.7,
        LetterGrade::BPlus => 3.3,
        LetterGrade::B => 3.0,
        LetterGrade::BMinus => 2.7,
        LetterGrade::CPlus => 2.3,
        LetterGrade::C => 2.0,
        LetterGrade::CMinus => 1.7,
        LetterGrade::DPlus => 1.3,
        LetterGrade::D => 1.0,
        LetterGrade::DMinus => 0.7,
        LetterGrade::F => 0.0,
    }
}

/*This is my pseudocode for the grading system
1. Use a vector to store the student's name and their corresponding grades
2. Create a function that will calculate the average grade of the student.
This function will take in the name of the student and the grade of the student and return the average grade of the student which will be the letter grade of the student.
3. Create a function that will calculate the GPA of the student. This function will take in the name of the student and the grade of the student.



 Create a function that will take in the student's name and their grades.
We will be using a vector to store the student`'s name and their grades.
A struct to type the student's name and their grades.

 */

// Percentage	GPA
// A+	97–100%	4.0
// A	93–96%	3.9
// A−	90–92%	3.7
// B+	87–89%	3.3
// B	83–86%	3.0
// B−	80–82%	2.7
// C+	77–79%	2.3
// C	73–76%	2.0
// C−	70–72%	1.7
// D+	67–69%	1.3
// D	63–66%	1.0
// D−	60–62%	0.7
// F	0–59%	0.0
