package db

const DbFilePathEnv = ""

type User struct {
	ID        int    `json:"id"`
	Lastname  string `json:"lastname"`
	Firstname string `json:"firstname"`
}

type Entreprises struct {
	ID        int    `json:"id"`
	UserID    int    `json:"user_id"`
	Name      string `json:"name"`
	Date      string `json:"date"`
	Codeape   string `json:"codeape"`
	Status    string `json:"status"`
	Jrsttx    int    `json:"jrsttx"`
	Jrsweek   int    `json:"jrsweek"`
	Jrsferies int    `json:"jrsferies"`
	Jrscp     int    `json:"jrscp"`
	Jan       int    `json:"jan"`
	Fev       int    `json:"fev"`
	Mar       int    `json:"mar"`
	Avr       int    `json:"avr"`
	Mai       int    `json:"mai"`
	Juin      int    `json:"juin"`
	Jui       int    `json:"jui"`
	Aout      int    `json:"aout"`
	Sept      int    `json:"sept"`
	Oct       int    `json:"oct"`
	Nov       int    `json:"nov"`
	Dec       int    `json:"dec"`
}

type Activites struct {
	ID              int     `json:"id"`
	UserId          int     `json:"user_id"`
	Production      int     `json:"production"`
	Entretien       int     `json:"entretien"`
	Clientele       int     `json:"clientele"`
	Interprofession int     `json:"interprofession"`
	Formation       int     `json:"formation"`
	Prodjour        int     `json:"prodjour"`
	TotalService    int     `json:"totalservice"`
	Tva             float32 `json:"tva"`
	Moyprix         float64 `json:"moyprix"`
	Donttva         float64 `json:"donttva"`
	Totalmoyprix    float64 `json:"totalmoyprix"`
	Htjours         float64 `json:"htjours"`
	Htcanann        float64 `json:"htcanann"`
	Tvaann          float64 `json:"tvaann"`
	Ttcann          float64 `json:"ttcann"`
}
