use gitql_parser::diagnostic::GQLError;

use termcolor::Color;

use crate::colored_stream::ColoredStream;

const PORPOT_LENGTH: usize = 6;

pub struct DiagnosticReporter {
    stdout: ColoredStream,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        Self {
            stdout: ColoredStream::new(),
        }
    }

    pub fn report_error(&mut self, message: &str) {
        self.stdout.set_color(Some(Color::Red));
        self.stdout.print("ERROR: ");
        self.stdout.println(message);
        self.stdout.reset();
    }

    pub fn report_gql_error(&mut self, error: GQLError) {
        self.stdout.set_color(Some(Color::Red));

        let start = error.location.start;
        self.stdout.print(&"-".repeat(PORPOT_LENGTH + start));
        self.stdout.println("^");

        self.stdout.print("Compiletime ERROR: ");

        let end = error.location.end;
        let message = error.message;
        self.stdout.print("[");
        self.stdout.print(&start.to_string());
        self.stdout.print(" - ");
        self.stdout.print(&end.to_string());
        self.stdout.print("] -> ");
        self.stdout.println(&message);

        self.stdout.reset();
    }

    pub fn report_runtime_error(&mut self, message: String) {
        self.stdout.set_color(Some(Color::Red));
        self.stdout.print("RUNTIME EXCEPTION: ");
        self.stdout.println(&message);

        self.stdout.reset();
    }
}
