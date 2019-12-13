package main

import (
	"database/sql"
	"fmt"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/labstack/echo"
)

type Entry struct {
	ID      int       `json:"id"`
	Date    time.Time `json:"date"`
	Content []string  `json:"content"`
}

type User struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

var (
	DB       *sql.DB
	EntryNum int64
)

const (
	YYYYMMDDhhmmss = "2006-01-02 15:04:05"
)

func createEntry(c echo.Context) error {
	d := time.Now()
	e := &Entry{}
	if err := c.Bind(e); err != nil {
		return err
	}
	_, err := DB.Exec(fmt.Sprintf("INSERT INTO entry(content, datetime) VALUES('%s', '%s')", e.Content, d.Format(YYYYMMDDhhmmss)))
	if err != nil {
		c.Logger().Error(err)
		return err
	}
	return c.JSON(http.StatusCreated, e)
}

func getEntries(c echo.Context) error {
	e := []Entry{}
	page, err := strconv.Atoi(c.Param("page"))
	if err != nil {
		c.Logger().Error(err)
		return err
	}
	rows, err := DB.Query(fmt.Sprintf("SELECT id, content, datetime from entry ORDER BY id DESC limit 10 OFFSET %d", (page-1)*10))
	if err != nil {
		c.Logger().Error(err)
	}
	defer rows.Close()
	for rows.Next() {
		ent := Entry{}
		var content string
		var id int
		err := rows.Scan(&id, &content, &ent.Date)
		if err != nil {
			c.Logger().Error(err)
			return err
		}
		ent.Content = strings.Split(content, "\n")
		e = append(e, ent)
	}

	var pages = make([]int, EntryNum/11+1)
	for index := range pages {
		pages[index] = index + 1
	}
	return c.JSON(http.StatusOK, e)
}

func createUser(c echo.Context) error {
	u := &User{
		ID: 1,
	}
	if err := c.Bind(u); err != nil {
		return err
	}
	// insert user
	return c.JSON(http.StatusCreated, u)
}

func deleteUser(c echo.Context) error {
	id, _ := strconv.Atoi(c.Param("id"))
	_, err := DB.Exec("DELETE from entry where id = ?", id)
	if err != nil {
		c.Logger().Error(err)
		return err
	}
	return c.NoContent(http.StatusNoContent)
}

func InitDB() error {
	address := os.Getenv("DBADDRESS")
	user := os.Getenv("DBUSER")
	pass := os.Getenv("DBPASSWORD")
	database := os.Getenv("DBNAME")
	dbport := os.Getenv("DBPORT")
	connstring := fmt.Sprintf("%s:%s@tcp(%s:%s)/%s?parseTime=true", user, pass, address, dbport, database)
	var err error
	DB, err = sql.Open("mysql", connstring)
	if err != nil {
		return err
	}
	fmt.Printf("%s/n", DB)
	fmt.Println("DB Connected")
	row, err := DB.Query("SELECT COUNT(*) FROM entry")
	if err != nil {
		return err
	}
	defer row.Close()
	row.Next()
	row.Scan(&EntryNum)
	return nil
}

func init() {
	err := InitDB()
	if err != nil {
		panic(err)
	}
}

func main() {
	e := echo.New()

	// Routing
	e.POST("/users", createUser)
	e.DELETE("/users", deleteUser)
	e.POST("/entries", createEntry)
	e.GET("/entries", getEntries)

	e.Logger.Fatal(e.Start(":9000"))

}
