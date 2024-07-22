quiz file MUST use LF line endings, not CRLF or anything else.

this is a general purpose quizzing program.
the quiz format is defined by three chunks:

<quiz name>\n
\n
<answer bank>\n
\n
<question pool>

quiz name is any string, this is the title of the quiz that will be displayed

answer bank is the list of all answers. not all answers need to be correct, these are randomly selected as phony options for questions
each answer is on its own line.
writing " / " (with the spaces) will act as a newline to assist with formatting the answer in the program.

question pool is a list of all questions.
each question is on its own line.
questions are of the format: <question>: <correct answer index>
question is any string and will be displayed as the prompt for that question
separating the prompt and the correct answer is ": ".
following this separator is the correct answer id. this is the 0-based index of the correct answer written in the answer pool.


when the quiz is loaded by the program, it will generate phony choices for questions randomly from the answer pool.
consequently, adding misleading answers is as simple as adding an entry to the answer pool. correctness is only determined by
the question entry pointing to a specific answer.

see q_codes.quiz.txt as an example.