#include <Arduino.h>

#define NUM_INNER 100
#define NUM_OUTER 100

int light = 0;
int interim_inner = 0;
int interim_outer[NUM_OUTER];
int outer = 0;

void setup() {
  Serial.begin(9600);

  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);

  for (int i = 0; i < NUM_OUTER; i++) {
    interim_outer[i] = analogRead(A6);
  }

  Serial.println("S");
}

void loop() {
  interim_inner = 0;
  for (int i = 0; i < NUM_INNER; i++) {
    interim_inner += analogRead(A6);
  }
  interim_outer[outer] = interim_inner / NUM_INNER;
  outer = (outer + 1) % NUM_OUTER;

  if (Serial.available() && Serial.read() == '?') {
    light = 0;
    for (int i = 0; i < NUM_OUTER; i++) {
      light += interim_outer[i];
    }
    Serial.print("L");
    Serial.println(light / NUM_OUTER);
  }
}
