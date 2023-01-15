package controllers

import (
	"fmt"
	"strings"
	"time"

	"github.com/aokabi/hobohi-diary/app"

	"github.com/revel/revel"
)

type App struct {
	*revel.Controller
}

type Entry struct {
	Id			int
	Day     time.Time
	Content []string
}

var (
	e       []Entry
	nowPage int
)

const (
	layout = "2006-01-02 15:04:05"
)

func (c App) Index(page int) revel.Result {
	e = []Entry{}
	revel.AppLog.Info("page = ", page)
	nowPage = page
	rows, err := app.DB.Query(fmt.Sprintf("SELECT id, content, datetime from entry ORDER BY id DESC limit 10 OFFSET %d", (page-1)*10))
	if err != nil {
		revel.AppLog.Info("DB Error", err)
	}
	defer rows.Close()
	for rows.Next() {
		ent := Entry{}
		var content string
		var id int
		err := rows.Scan(&id, &content, &ent.Day)
		if err != nil {
			revel.AppLog.Info("error",err)
		}
		ent.Content = strings.Split(content, "\n")
		e = append(e, ent)
	}
	c.ViewArgs["entry"] = e

	var pages = make([]int, app.EntryNum/11+1)
	for index := range pages {
		pages[index] = index + 1
	}
	c.ViewArgs["page"] = pages
	return c.Render()
}

func (c App) Post() revel.Result {
	//revel.AppLog.Info(c.Request)
	d := time.Now()
	_, err := app.DB.Exec(fmt.Sprintf("INSERT INTO entry(content, datetime) VALUES('%s', '%s')", c.Params.Form.Get("content"), d.Format(layout)))
	if err != nil {
		revel.AppLog.Info("DB error", err)
	}
	return c.Redirect(App.Index, nowPage)
}
