package main

import (
	"errors"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"

	memless "github.com/nascent-tech/memless/bindings/go"
)

func main() {
	if len(os.Args) >= 4 {
		fmt.Print(writeDispatch(os.Args[1], os.Args[2], os.Args[3]))
		return
	}
	if len(os.Args) > 2 {
		fmt.Print(queryOutcome(os.Args[1], os.Args[2]))
		return
	}
	fmt.Println(loadOutcome(argPath()))
}

var writeModes = map[string]func(string, string) string{
	"write-disk":       writeDiskOutcome,
	"transaction":      transactionOutcome,
	"transaction-disk": transactionDiskOutcome,
	"reload":           reloadOutcome,
}

func writeDispatch(path string, sql string, mode string) string {
	if outcome, known := writeModes[mode]; known {
		return outcome(path, sql)
	}
	return writeOutcome(path, sql)
}

func argPath() string {
	if len(os.Args) <= 1 {
		return ""
	}
	return os.Args[1]
}

func loadOutcome(path string) string {
	instance, err := memless.Load(path)
	if err == nil {
		instance.Release()
		return "accepted"
	}
	return errorLine(err)
}

func queryOutcome(path string, sql string) string {
	instance, err := memless.Load(path)
	if err != nil {
		return errorLine(err) + "\n"
	}
	defer instance.Release()
	rows, err := instance.Query(sql)
	if err != nil {
		return errorLine(err) + "\n"
	}
	return render(rows)
}

func errorLine(err error) string {
	var refusal *memless.RefusalError
	if errors.As(err, &refusal) {
		return "refused: " + refusal.Message
	}
	return "fault: " + err.Error()
}

func render(rows memless.Rows) string {
	if len(rows.Rows) == 0 {
		return "ok\nempty\n"
	}
	var out strings.Builder
	out.WriteString("ok\ncols")
	writeCells(&out, rows.Columns)
	out.WriteString("\n")
	writeRows(&out, rows.Rows)
	return out.String()
}

func writeCells(out *strings.Builder, columns []string) {
	for _, name := range columns {
		out.WriteString("\t")
		out.WriteString(escape(name))
	}
}

func writeRows(out *strings.Builder, rows [][]any) {
	for _, row := range rows {
		out.WriteString("row")
		writeRow(out, row)
		out.WriteString("\n")
	}
}

func writeRow(out *strings.Builder, row []any) {
	for _, value := range row {
		out.WriteString("\t")
		out.WriteString(renderCell(value))
	}
}

func renderCell(value any) string {
	if value == nil {
		return "null"
	}
	if number, ok := value.(int64); ok {
		return "int:" + strconv.FormatInt(number, 10)
	}
	if number, ok := value.(float64); ok {
		return fmt.Sprintf("dec:%016x", math.Float64bits(number))
	}
	if flag, ok := value.(bool); ok {
		return renderBool(flag)
	}
	return "str:" + escape(value.(string))
}

func renderBool(flag bool) string {
	if flag {
		return "bool:true"
	}
	return "bool:false"
}

func escape(text string) string {
	text = strings.ReplaceAll(text, "\\", "\\\\")
	text = strings.ReplaceAll(text, "\t", "\\t")
	return strings.ReplaceAll(text, "\n", "\\n")
}
