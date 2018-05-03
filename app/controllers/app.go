package controllers

import (
	"time"

	"github.com/revel/revel"
)

type App struct {
	*revel.Controller
}

type entry struct {
	Day     time.Time
	Content string
}

var (
	e []entry
)

const (
	layout = "2006-01-02"
)

func (c App) Index() revel.Result {
	return c.Render()
}

func (c App) Post() revel.Result {
	revel.AppLog.Debug(c.Params.Form.Get("content"))
	d := time.Now()
	if len(e) == 0 {
		e = append(e, entry{d, c.Params.Form.Get("content")})
	} else {
		e = append([]entry{entry{d, c.Params.Form.Get("content")}}, e...)
	}
	c.ViewArgs["entry"] = e
	return c.RenderTemplate("App/Index.html")
}
