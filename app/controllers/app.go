package controllers

import (
	"fmt"
	"strings"
	"time"

	"diary/app"

	"github.com/revel/revel"
)

type App struct {
	*revel.Controller
}

type Entry struct {
	Day     time.Time
	Content []string
}

var (
	e []Entry
)

const (
	layout = "2006-01-02 15:04:05"
)

func (c App) Index(page int) revel.Result {
	e = []Entry{}
	revel.INFO.Println(page)
	rows, err := app.DB.Query(fmt.Sprintf("SELECT content, datetime from entry ORDER BY id DESC limit 10 OFFSET %d", (page-1)*10))
	if err != nil {
		revel.INFO.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		ent := Entry{}
		var content string
		err := rows.Scan(&content, &ent.Day)
		if err != nil {
			revel.INFO.Println(err)
		}
		ent.Content = strings.Split(content, "\n")
		e = append(e, ent)
	}
	c.ViewArgs["entry"] = e
	return c.Render()
}

func (c App) Post() revel.Result {
	d := time.Now()
	_, err := app.DB.Exec(fmt.Sprintf("INSERT INTO entry(content, datetime) VALUES('%s', '%s')", c.Params.Form.Get("content"), d.Format(layout)))
	if err != nil {
		revel.INFO.Println(err)
	}
	return c.Redirect(App.Index)
}
