use calamine::{Reader, Xlsx, open_workbook, XlsxError};

pub fn read_excel_to_vec(path: &str, worksheet_name: &str) -> Result<Vec<Vec<String>>, String> {
    let mut res: Vec<Vec<String>> = Vec::new();

    let mut excel: Xlsx<_> = open_workbook(path).map_err(|x: XlsxError| format!("failed to open '{}': {}", path, x))?;

    if let Some(Ok(r)) = excel.worksheet_range(worksheet_name) {
        for row in r.rows() {
            let r: Vec<String> = row.iter().map(|x| format!("{}", x)).collect();
            res.push(r);
        }
    } else {
        return Err(format!("Failed to find '{}' worksheet in the book", worksheet_name));
    }

    Ok(res)
}
