use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use std::fs::File;
use std::io::BufWriter;
use crate::models::TransportGroup;

pub fn generate_pdf(allocations: &[TransportGroup], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    // --- PDF SETUP ---
    let (doc, page1, layer1) = PdfDocument::new("Transport Sheet", Mm(297.0), Mm(210.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    // --- FONTS ---
    let font      = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    // --- LAYOUT SETTINGS ---
    let mut x_cursor = Mm(10.0);
    let start_y = Mm(170.0); 
    let col_width = Mm(45.0);
    let row_height = Mm(7.0);

    // For each car / minibus on the created transport sheet
    for group in allocations {

        // Collecting preset colours for locations
        let (r, g, b) = group.destination.color_rgb();
        
        // --- DRAWING THE HEADER BOX ---
        let rect_points = vec![
            (Point::new(x_cursor, start_y), false),
            (Point::new(x_cursor + col_width, start_y), false),
            (Point::new(x_cursor + col_width, start_y + Mm(20.0)), false),
            (Point::new(x_cursor, start_y + Mm(20.0)), false),
        ];
        
        // In printpdf 0.7.0, Polygon directly takes the points in 'rings'
        let rect_poly = Polygon {
            rings: vec![rect_points],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::EvenOdd,
        };

        // Creating a black outline for the header box and filling in the box with the preset location colour
        current_layer.set_fill_color(Color::Rgb(Rgb::new(r, g, b, None)));
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(0.5);

        // Adding the header rectangle to the PDF layer
        current_layer.add_polygon(rect_poly);
        
        // --- HEADER TEXT ---
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.use_text(group.departure_time.clone(), 10.0, Mm(x_cursor.0 + 2.0), Mm(start_y.0 + 15.0), &font_bold);
        current_layer.use_text(group.destination.label(), 10.0, Mm(x_cursor.0 + 2.0), Mm(start_y.0 + 11.0), &font);
        
        current_layer.use_text(format!("Driver: {}", group.driver.name), 9.0, Mm(x_cursor.0 + 2.0), Mm(start_y.0 + 6.0), &font_bold);
        current_layer.use_text(&group.vehicle_label, 8.0, Mm(x_cursor.0 + 2.0), Mm(start_y.0 + 2.0), &font);

        // --- PASSENGERS ---
        let mut y_cursor = start_y; 
        
        for p in &group.passengers {
            y_cursor -= row_height;
            
            let cell_points = vec![
                (Point::new(x_cursor, y_cursor), false),
                (Point::new(x_cursor + col_width, y_cursor), false),
                (Point::new(x_cursor + col_width, y_cursor + row_height), false),
                (Point::new(x_cursor, y_cursor + row_height), false),
            ];

            let cell_poly = Polygon {
                rings: vec![cell_points],
                mode: PaintMode::FillStroke,
                winding_order: WindingOrder::EvenOdd,
            };

            current_layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None))); 
            current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
            
            current_layer.add_polygon(cell_poly);

            current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
            current_layer.use_text(&p.name, 10.0, Mm(x_cursor.0 + 2.0), Mm(y_cursor.0 + 2.0), &font);
        }

        x_cursor += col_width;

        if x_cursor.0 > 250.0 {
            x_cursor = Mm(10.0);
        }
    }

    doc.save(&mut BufWriter::new(File::create(filename)?)).map_err(|e| e.into())
}