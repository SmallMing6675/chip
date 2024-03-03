
import random

def generate_string(length_kb):
  """Generates a string with repetitions to potentially be compressed with LZ77.

  Args:
      length_kb: The desired length of the string in kilobytes.

  Returns:
      A string with the specified length containing some repeated phrases.
  """

  # Choose some common words/phrases
  phrases = ["This is a test string", "with some repetition", "of common phrases"]

  # Construct the string with repetitions and random variations
  string = ""
  while len(string) // 1024 < length_kb:
    string += random.choice(phrases) + " "
    string += " ".join(random.choice(phrases) for _ in range(2, 5)) + " "

  return string

# Example usage
string = generate_string(100)
print(string)  # Print a snippet of the generated string
