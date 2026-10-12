// Esperado: [ERRO LÉXICO] string não terminada na linha 3

int main(void) {
  char *s;
  s = "unterminated;
  return 0;
}
