// Esperado: [ERRO SEMÂNTICO] Variável 'total' utilizada na linha 4 não foi
// declarada previamente.

int main(void) {
  int x;
  x = 10;
  total = x + 5;
  return 0;
}
