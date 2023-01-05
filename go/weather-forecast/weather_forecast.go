// Package weather for Goblinocus.
package weather

// CurrentCondition variable.
var CurrentCondition string

// CurrentLocation variable.
var CurrentLocation string

// Forecast function.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
