extern void write(char);

int main() {
  const char *s = "Hello!";
  while (s++) {
    write(*s);
  }
  return 0;
}